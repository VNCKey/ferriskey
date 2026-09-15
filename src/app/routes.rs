#[derive(PartialEq, Eq, Hash, Default, Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum AppRoute {
    #[default]
    LandingPage,
    TutorialCargo,
    Comenzando,
    LibTiposDatos,
    TutorialCompilacion,
    TutorialTiposDatos,
    TutorialControlFlujo,
    TutorialFunciones,
    TutorialIteradores,
    TutorialOwnership,
    TutorialStructs,
    TutorialEnums,
    TutorialColecciones,
    TutorialErrores,
    TutorialTraits,
    TutorialGenericos,
    TutorialStrings,
    TutorialMemoria,
    TutorialModulos,
    DashboardGraficos,
    Playground,
    PlaygroundNube,
}
