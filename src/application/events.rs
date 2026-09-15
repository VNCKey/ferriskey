use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;

use crate::app::routes::AppRoute;

/// Intenciones / Comandos de la UI dirigidos a workers en segundo plano.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AppCommand {
    EjecutarCodigo { route: AppRoute, code: String },
    ExpandirMacros { code: String },
    AnalizarBinario { path: PathBuf },
}

/// Eventos emitidos por los workers asíncronos para actualizar el estado de la UI.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AppEvent {
    CodigoEjecutado { route: AppRoute, output: String },
    MacroExpandida { result: String },
    BinarioAnalizado { report: String },
    ErrorOcurrido { message: String },
}

/// Bus de comunicación desacoplado entre la GUI egui y las tareas asíncronas de fondo.
pub struct EventBus {
    pub tx_command: Sender<AppCommand>,
    pub rx_event: Receiver<AppEvent>,
}

impl EventBus {
    pub fn new() -> (Self, Sender<AppEvent>, Receiver<AppCommand>) {
        let (tx_cmd, rx_cmd) = channel::<AppCommand>();
        let (tx_evt, rx_evt) = channel::<AppEvent>();

        let bus = Self {
            tx_command: tx_cmd,
            rx_event: rx_evt,
        };

        (bus, tx_evt, rx_cmd)
    }

    pub fn enviar_comando(&self, command: AppCommand) -> Result<(), String> {
        self.tx_command
            .send(command)
            .map_err(|e| format!("No se pudo enviar el comando: {e}"))
    }

    pub fn procesar_eventos<F>(&self, mut handler: F)
    where
        F: FnMut(AppEvent),
    {
        while let Ok(event) = self.rx_event.try_recv() {
            handler(event);
        }
    }
}

/// Worker asíncrono en segundo plano que escucha comandos y ejecuta procesos de infraestructura.
pub fn iniciar_worker_asincrono(rx_cmd: Receiver<AppCommand>, tx_evt: Sender<AppEvent>) {
    thread::spawn(move || {
        while let Ok(cmd) = rx_cmd.recv() {
            match cmd {
                AppCommand::EjecutarCodigo { route, code } => {
                    let output = crate::infrastructure::execution::ejecutar_codigo_rust(&code);
                    let _ = tx_evt.send(AppEvent::CodigoEjecutado { route, output });
                }
                AppCommand::ExpandirMacros { code } => {
                    let result = crate::infrastructure::execution::expandir_macros_rust(&code);
                    let _ = tx_evt.send(AppEvent::MacroExpandida { result });
                }
                AppCommand::AnalizarBinario { path } => {
                    match crate::infrastructure::binary_analysis::render_report(&path, crate::infrastructure::binary_analysis::AnalysisFocus::Header) {
                        Ok(report) => {
                            let _ = tx_evt.send(AppEvent::BinarioAnalizado { report });
                        }
                        Err(err) => {
                            let _ = tx_evt.send(AppEvent::ErrorOcurrido { message: err });
                        }
                    }
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_event_bus_command_flow() {
        let (bus, tx_evt, rx_cmd) = EventBus::new();
        iniciar_worker_asincrono(rx_cmd, tx_evt);

        let res = bus.enviar_comando(AppCommand::EjecutarCodigo {
            route: AppRoute::Playground,
            code: "fn main() { println!(\"Test EventBus\"); }".to_string(),
        });
        assert!(res.is_ok());

        // Esperar la respuesta del worker mediante recv_timeout robusco
        let event = bus.rx_event.recv_timeout(Duration::from_secs(10));
        assert!(event.is_ok());
        if let Ok(AppEvent::CodigoEjecutado { route, output }) = event {
            assert_eq!(route, AppRoute::Playground);
            assert!(output.contains("Test EventBus"));
        } else {
            panic!("Respuesta de evento inesperada");
        }
    }
}
