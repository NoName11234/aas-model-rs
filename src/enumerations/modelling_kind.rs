///Enumeration for denoting whether an element is a template or an instance.
#[derive(PartialEq, Clone)]
pub enum ModellingKind {
    ///Specification of the common features of a structured element in sufficient detail that such
    /// an instance can be instantiated using it.
    Template,
    ///Concrete, cleary identifiable element instance. Its creation and validation may be guided by
    /// a corresponding element template.
    Instance
}