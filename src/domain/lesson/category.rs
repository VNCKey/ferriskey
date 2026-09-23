use serde::{Deserialize, Serialize};

/// Categorías pedagógicas en las que se divide el plan de estudios de FerrisKey.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum LessonCategory {
    #[default]
    Playground,
    Basics,
    ControlFlow,
    OwnershipAndMemory,
    CustomTypes,
    Collections,
    ErrorHandling,
    TraitsAndGenerics,
    FunctionsAndClosures,
    Iterators,
    ModulesAndVisibility,
    Concurrency,
    Macros,
    Advanced,
}

impl LessonCategory {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Playground => "Playground & Experimentación",
            Self::Basics => "Conceptos Básicos & Tipos Primitivos",
            Self::ControlFlow => "Control de Flujo",
            Self::OwnershipAndMemory => "Ownership & Gestión de Memoria",
            Self::CustomTypes => "Tipos Personalizados (Structs & Enums)",
            Self::Collections => "Colecciones Estándar",
            Self::ErrorHandling => "Manejo de Errores",
            Self::TraitsAndGenerics => "Traits & Polimorfismo Genérico",
            Self::FunctionsAndClosures => "Funciones & Closures",
            Self::Iterators => "Iteradores & Programación Funcional",
            Self::ModulesAndVisibility => "Módulos & Organización",
            Self::Concurrency => "Concurrencia Segura",
            Self::Macros => "Metaprogramación & Macros",
            Self::Advanced => "Rust Avanzado & FFI",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Playground => "play",
            Self::Basics => "code",
            Self::ControlFlow => "fork",
            Self::OwnershipAndMemory => "memory",
            Self::CustomTypes => "box",
            Self::Collections => "list",
            Self::ErrorHandling => "shield",
            Self::TraitsAndGenerics => "puzzle",
            Self::FunctionsAndClosures => "function",
            Self::Iterators => "repeat",
            Self::ModulesAndVisibility => "folder",
            Self::Concurrency => "cpu",
            Self::Macros => "sparkles",
            Self::Advanced => "wrench",
        }
    }
}
