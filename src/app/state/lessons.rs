use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::app::routes::AppRoute;

/// Datos individuales de una lección ejecutable (código fuente y salida de la terminal).
#[derive(Clone, Debug)]
pub struct LessonData {
    pub id: String,
    pub title: String,
    pub code: String,
    pub output: Arc<Mutex<String>>,
}

impl LessonData {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        initial_code: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            code: initial_code.into(),
            output: Arc::new(Mutex::new(String::new())),
        }
    }

    pub fn limpiar_salida(&self) {
        if let Ok(mut out) = self.output.lock() {
            out.clear();
        }
    }
}

/// Catálogo centralizado de lecciones gestionado mediante un registro modular.
#[derive(Default, Debug)]
pub struct LessonCatalog {
    lessons: HashMap<AppRoute, LessonData>,
}

impl LessonCatalog {
    pub fn new() -> Self {
        let mut catalog = Self {
            lessons: HashMap::new(),
        };

        catalog.registrar(
            AppRoute::Playground,
            "Playground Local",
            crate::content::PLAYGROUND_CODE,
        );
        catalog.registrar(
            AppRoute::PlaygroundNube,
            "Playground Nube",
            crate::content::PLAYGROUND_NUBE_CODE,
        );
        catalog.registrar(
            AppRoute::TutorialTiposDatos,
            "Tipos de Datos",
            crate::content::DATATYPES_CODE,
        );
        catalog.registrar(
            AppRoute::TutorialControlFlujo,
            "Control de Flujo",
            crate::content::CONTROL_FLUJO_CODE,
        );
        catalog.registrar(
            AppRoute::TutorialOwnership,
            "Ownership",
            crate::content::OWNERSHIP_CODE,
        );
        catalog.registrar(
            AppRoute::TutorialMemoria,
            "Memoria",
            crate::content::OWNERSHIP_CODE,
        );
        catalog.registrar(
            AppRoute::TutorialStrings,
            "Strings",
            crate::content::OWNERSHIP_CODE,
        );
        catalog.registrar(
            AppRoute::TutorialStructs,
            "Structs",
            crate::content::STRUCTS_CODE,
        );
        catalog.registrar(AppRoute::TutorialEnums, "Enums", crate::content::ENUMS_CODE);
        catalog.registrar(
            AppRoute::TutorialColecciones,
            "Colecciones",
            crate::content::COLLECTIONS_CODE,
        );
        catalog.registrar(
            AppRoute::TutorialErrores,
            "Error Handling",
            crate::content::ERRORS_CODE,
        );
        catalog.registrar(
            AppRoute::TutorialTraits,
            "Traits",
            crate::content::TRAITS_CODE,
        );
        catalog.registrar(
            AppRoute::TutorialGenericos,
            "Generics",
            crate::content::GENERICS_CODE,
        );
        catalog.registrar(
            AppRoute::TutorialFunciones,
            "Funciones & Closures",
            crate::content::FUNCTIONS_CODE,
        );
        catalog.registrar(
            AppRoute::TutorialIteradores,
            "Iteradores",
            crate::content::ITERATORS_CODE,
        );
        catalog.registrar(
            AppRoute::TutorialModulos,
            "Modules & Visibility",
            crate::content::MODULES_CODE,
        );

        catalog
    }

    pub fn registrar(
        &mut self,
        route: AppRoute,
        title: impl Into<String>,
        code: impl Into<String>,
    ) {
        self.lessons
            .insert(route, LessonData::new(format!("{:?}", route), title, code));
    }

    pub fn obtener(&self, route: AppRoute) -> Option<&LessonData> {
        self.lessons.get(&route)
    }

    pub fn obtener_mut(&mut self, route: AppRoute) -> Option<&mut LessonData> {
        self.lessons.get_mut(&route)
    }
}

pub struct LessonsState {
    pub catalog: LessonCatalog,

    pub playground_code: String,
    pub playground_output: Arc<Mutex<String>>,
    pub datatypes_code: String,
    pub datatypes_output: Arc<Mutex<String>>,
    pub playground_nube_code: String,
    pub playground_nube_output: Arc<Mutex<String>>,

    pub anim_compilacion_activa: bool,
    pub compilacion_progreso: f32,
    pub compilacion_etapa_seleccionada: usize,

    pub controlflujo_code: String,
    pub controlflujo_output: Arc<Mutex<String>>,
    pub ownership_code: String,
    pub ownership_output: Arc<Mutex<String>>,
    pub strings_ownership_tab: usize,
    pub structs_code: String,
    pub structs_output: Arc<Mutex<String>>,
    pub enums_code: String,
    pub enums_output: Arc<Mutex<String>>,
    pub colecciones_code: String,
    pub colecciones_output: Arc<Mutex<String>>,
    pub vec_sim_len: usize,
    pub vec_sim_cap: usize,
    pub colecciones_tab: usize,
    pub errores_code: String,
    pub errores_output: Arc<Mutex<String>>,
    pub err_pipeline_fail: bool,
    pub errores_tab: usize,
    pub errores_preguntas_respuestas: [Option<usize>; 5],
    pub traits_code: String,
    pub traits_output: Arc<Mutex<String>>,
    pub genericos_code: String,
    pub genericos_output: Arc<Mutex<String>>,
    pub genericos_preguntas_respuestas: [Option<usize>; 5],

    pub arr_elem_type: usize,
    pub arr_len: usize,
    pub arr_active_idx: usize,
    pub arr_action_msg: String,
    pub compuestos_tab: usize,
    pub slice_start: usize,
    pub slice_end: usize,
    pub tup_t0: usize,
    pub tup_t1: usize,
    pub tup_t2: usize,
    pub structs_tab: usize,
    pub structs_preguntas_respuestas: [Option<usize>; 5],

    pub funciones_code: String,
    pub funciones_output: Arc<Mutex<String>>,
    pub funciones_tab: usize,
    pub funciones_preguntas_respuestas: [Option<usize>; 5],
    pub iteradores_tab: usize,
    pub iteradores_preguntas_respuestas: [Option<usize>; 5],
    pub enums_tab: usize,
    pub traits_tab: usize,
    pub genericos_tab: usize,
    pub iteradores_code: String,
    pub iteradores_output: Arc<Mutex<String>>,
    pub iter_mode: usize,
    pub iter_filter_even: bool,
    pub arr_code: String,
    pub arr_output: Arc<Mutex<String>>,
    pub slice_code: String,
    pub slice_output: Arc<Mutex<String>>,
    pub tup_code: String,
    pub tup_output: Arc<Mutex<String>>,
    pub compuestos_info_tab: usize,

    pub pilares_step: usize,
    pub anatomy_step: usize,
    pub codelab_reto_actual: usize,
    pub codelab_retos_completados: [bool; 6],
    pub codelab_respuestas: [Option<usize>; 5],
    pub tipo_primitivo_categoria: usize,
    pub tipo_entero_seleccionado: usize,
    pub conceptos_enteros_familia: usize,
    pub estructura_code: String,
    pub estructura_output: Arc<Mutex<String>>,
    pub estructura_tab: usize,
    pub conceptos_tab: usize,
    pub conceptos_codelab_reto_actual: usize,
    pub conceptos_preguntas_respuestas: [Option<usize>; 15],
    pub controlflujo_tab: usize,
    pub controlflujo_is_practica: bool,
    pub controlflujo_preguntas_respuestas: [Option<usize>; 10],
    pub conceptos_code: String,
    pub conceptos_output: Arc<Mutex<String>>,
    pub modulos_code: String,
    pub modulos_output: Arc<Mutex<String>>,
    pub modulos_tab: usize,
    pub modulos_preguntas_respuestas: [Option<usize>; 5],
    pub compuestos_preguntas_respuestas: [Option<usize>; 10],
    pub session_codelab_reto: usize,
}

impl LessonsState {
    pub fn obtener_codigo(&self, route: AppRoute) -> Option<&str> {
        self.catalog.obtener(route).map(|l| l.code.as_str())
    }

    pub fn obtener_editor_mut(
        &mut self,
        route: AppRoute,
    ) -> Option<(&mut String, Arc<Mutex<String>>)> {
        if let Some(lesson) = self.catalog.obtener_mut(route) {
            let output = Arc::clone(&lesson.output);
            Some((&mut lesson.code, output))
        } else {
            None
        }
    }
}

impl Default for LessonsState {
    fn default() -> Self {
        Self {
            catalog: LessonCatalog::new(),
            playground_code: crate::content::PLAYGROUND_CODE.to_owned(),
            playground_output: Arc::new(Mutex::new(String::new())),
            datatypes_code: String::new(),
            datatypes_output: Arc::new(Mutex::new(String::new())),
            playground_nube_code: crate::content::PLAYGROUND_NUBE_CODE.to_owned(),
            playground_nube_output: Arc::new(Mutex::new(String::new())),

            anim_compilacion_activa: false,
            compilacion_progreso: 1.0,
            compilacion_etapa_seleccionada: 4,
            controlflujo_code: String::new(),
            controlflujo_output: Arc::new(Mutex::new(String::new())),
            ownership_code: crate::content::OWNERSHIP_CODE.to_owned(),
            ownership_output: Arc::new(Mutex::new(String::new())),
            strings_ownership_tab: 0,
            structs_code: String::new(),
            structs_output: Arc::new(Mutex::new(String::new())),
            enums_code: crate::content::ENUMS_CODE.to_owned(),
            enums_output: Arc::new(Mutex::new(String::new())),
            colecciones_code: crate::content::COLLECTIONS_CODE.to_owned(),
            colecciones_output: Arc::new(Mutex::new(String::new())),
            vec_sim_len: 3,
            vec_sim_cap: 4,
            colecciones_tab: 0,
            errores_code: String::new(),
            errores_output: Arc::new(Mutex::new(String::new())),
            err_pipeline_fail: false,
            errores_tab: 0,
            errores_preguntas_respuestas: [None; 5],
            traits_code: crate::content::TRAITS_CODE.to_owned(),
            traits_output: Arc::new(Mutex::new(String::new())),
            genericos_code: String::new(),
            genericos_output: Arc::new(Mutex::new(String::new())),
            genericos_preguntas_respuestas: [None; 5],
            genericos_tab: 0,
            arr_elem_type: 1,
            arr_len: 5,
            arr_active_idx: 2,
            arr_action_msg: "Inspecciona métodos y accesos a elementos del arreglo".to_string(),
            compuestos_tab: 0,
            slice_start: 1,
            slice_end: 4,
            tup_t0: 1,
            tup_t1: 3,
            tup_t2: 2,
            structs_tab: 0,
            structs_preguntas_respuestas: [None; 5],
            funciones_code: String::new(),
            funciones_output: Arc::new(Mutex::new(String::new())),
            funciones_tab: 0,
            funciones_preguntas_respuestas: [None; 5],
            iteradores_tab: 0,
            enums_tab: 0,
            traits_tab: 0,
            iteradores_code: String::new(),
            iteradores_output: Arc::new(Mutex::new(String::new())),
            iteradores_preguntas_respuestas: [None; 5],
            iter_mode: 0,
            iter_filter_even: true,
            arr_code: crate::content::ARRAYS_CODE.to_owned(),
            arr_output: Arc::new(Mutex::new(String::new())),
            slice_code: crate::content::SLICES_CODE.to_owned(),
            slice_output: Arc::new(Mutex::new(String::new())),
            tup_code: crate::content::TUPLES_CODE.to_owned(),
            tup_output: Arc::new(Mutex::new(String::new())),
            compuestos_info_tab: 0,
            pilares_step: 0,
            anatomy_step: 0,
            codelab_reto_actual: 0,
            codelab_retos_completados: [false, false, false, false, false, false],
            codelab_respuestas: [None; 5],
            tipo_primitivo_categoria: 0,
            tipo_entero_seleccionado: 2,
            conceptos_enteros_familia: 0,
            estructura_code: String::new(),
            estructura_output: Arc::new(Mutex::new(String::new())),
            estructura_tab: 0,
            conceptos_tab: 7,
            conceptos_codelab_reto_actual: 0,
            conceptos_preguntas_respuestas: [None; 15],
            controlflujo_tab: 0,
            controlflujo_is_practica: false,
            controlflujo_preguntas_respuestas: [None; 10],
            conceptos_code: String::new(),
            conceptos_output: Arc::new(Mutex::new(String::new())),
            modulos_code: crate::content::MODULES_CODE.to_owned(),
            modulos_output: Arc::new(Mutex::new(String::new())),
            modulos_tab: 0,
            modulos_preguntas_respuestas: [None; 5],
            compuestos_preguntas_respuestas: [None; 10],
            session_codelab_reto: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lesson_catalog_initialization() {
        let catalog = LessonCatalog::new();
        assert!(catalog.obtener(AppRoute::TutorialControlFlujo).is_some());
        let lesson = catalog.obtener(AppRoute::TutorialControlFlujo).unwrap();
        assert_eq!(lesson.title, "Control de Flujo");
        assert!(!lesson.code.is_empty());
    }

    #[test]
    fn test_lessons_state_obtener_codigo() {
        let state = LessonsState::default();
        let code = state.obtener_codigo(AppRoute::TutorialControlFlujo);
        assert!(code.is_some());
    }
}
