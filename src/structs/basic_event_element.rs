use serde::{Deserialize, Serialize};

use crate::enumerations::direction::Direction;
use crate::enumerations::interface_enumerations::referable::Referable;
use crate::enumerations::state_of_event::StateOfEvent;
use crate::structs::extension::Extension;
use crate::structs::multi_language_name_type::MultiLanguageNameType;
use crate::structs::multi_language_text_type::MultiLanguageTextType;
use crate::structs::qualifier::Qualifier;
use crate::structs::reference::Reference;
use crate::traits::event_element::TEventElement;
use crate::traits::has_data_specification::THasDataSpecification;
use crate::traits::has_extensions::THasExtensions;
use crate::traits::has_semantics::THasSemantics;
use crate::traits::qualifiable::TQualifiable;
use crate::traits::referable::TReferable;
use crate::traits::submodel_element::TSubmodelElement;

///A basic event element.
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct BasicEventElement {
    ///Reference to a referable, e.g. a data element or a submodel that is being observed.
    observed: Reference,
    ///The direction of the event.
    direction: Direction,
    ///The state of the event.
    state: StateOfEvent,
    ///Information for the outer message infrastructure to schedule the event for the respective
    /// communication channel.
    #[serde(rename = "messageTopic")]
    message_topic: Option<String>,
    ///Information about which outer message infrastructure shall handle messages for the event
    /// element.
    #[serde(rename = "messageBroker")]
    message_broker: Option<Referable>,
    ///Optional timestamp in UTC when the last event was received or sent.
    #[serde(rename = "lastUpdate")]
    last_update: Option<String>,
    ///Optional maximum frequency the software entity behind the referable/the outer infrastructure
    /// can handle events.
    #[serde(rename = "minInterval")]
    min_interval: Option<String>,
    ///Optional maximum output interval.
    #[serde(rename = "maxInterval")]
    max_interval: Option<String>,
    category: Option<String>,
    #[serde(rename = "idShort")]
    id_short: Option<String>,
    #[serde(rename = "displayName")]
    display_name: Vec<MultiLanguageNameType>,
    description: Vec<MultiLanguageTextType>,
    extensions: Vec<Extension>,
    #[serde(rename = "semanticId")]
    semantic_id: Option<Reference>,
    #[serde(rename = "supplementalSemanticIds")]
    supplemental_semantic_ids: Vec<Reference>,
    qualifiers: Vec<Qualifier>,
    specifications: Vec<Reference>
}

impl BasicEventElement {
    ///Creates a new instance of the struct.
    ///
    /// [observed]: reference to referable being observed
    /// [direction]: direction of the event
    /// [state]: state of the event
    pub fn new(observed: Reference, direction: Direction, state: StateOfEvent) -> BasicEventElement {
        BasicEventElement {
            observed,
            direction,
            state,
            message_topic: None,
            message_broker: None,
            last_update: None,
            min_interval: None,
            max_interval: None,
            category: None,
            id_short: None,
            display_name: Vec::new(),
            description: Vec::new(),
            extensions: Vec::new(),
            semantic_id: None,
            supplemental_semantic_ids: Vec::new(),
            qualifiers: Vec::new(),
            specifications: Vec::new()
        }
    }

    ///Sets the reference to the referable being observed.
    /// [observed]: reference to referable
    pub fn set_observed(&mut self, observed: Reference) {
        self.observed = observed;
    }

    ///Returns the reference to the referable being observed.
    pub fn get_observed(&self) -> &Reference {
        &self.observed
    }

    ///Returns the mutable reference to the referable being observed.
    pub fn get_mut_observed(&mut self) -> &mut Reference {
        &mut self.observed
    }

    ///Sets the direction of the event.
    ///
    /// [direction]: direction of event
    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    ///Returns the direction of the event.
    pub fn get_direction(&self) -> &Direction {
        &self.direction
    }

    ///Returns the mutable direction of the event.
    pub fn get_mut_direction(&mut self) -> &mut Direction {
        &mut self.direction
    }

    ///Sets the state of the event.
    ///
    /// [state]: state of event
    pub fn set_state(&mut self, state: StateOfEvent) {
        self.state = state;
    }

    ///Returns the state of the event.
    pub fn get_state(&self) -> &StateOfEvent {
        &self.state
    }

    ///Returns the mutable state of the event.
    pub fn get_mut_state(&mut self) -> &mut StateOfEvent {
        &mut self.state
    }

    ///Sets the information for the outer message infrastructure.
    ///
    /// [topic]: information for message infrastructure
    pub fn set_message_topic(&mut self, topic: String) {
        self.message_topic = Some(topic);
    }

    ///Returns the information for the outer message infrastructure.
    pub fn get_message_topic(&self) -> Option<&String> {
        self.message_topic.as_ref()
    }

    ///Returns the mutable information for the outer message infrastructure.
    pub fn get_mutable_message_topic(&mut self) -> Option<&mut String> {
        self.message_topic.as_mut()
    }

    ///Sets the information about which outer message broker shall be used.
    ///
    /// [message_broker]: information about message broker
    pub fn set_message_broker(&mut self, message_broker: Referable) {
        self.message_broker = Some(message_broker);
    }

    ///Returns the information about which outer message broker shall be used.
    pub fn get_message_broker(&self) -> Option<&Referable> {
        self.message_broker.as_ref()
    }

    ///Returns the mutable information about which outer message broker shall be used.
    pub fn get_mut_message_broker(&mut self) -> Option<&mut Referable> {
        self.message_broker.as_mut()
    }

    ///Sets the timestamp when the last event was received or sent.
    ///
    /// [timestamp]: timestamp
    pub fn set_last_update(&mut self, last_update: String) {
        self.last_update = Some(last_update);
    }

    ///Returns the timestamp when the last event was received or sent.
    pub fn get_last_update(&self) -> Option<&String> {
        self.last_update.as_ref()
    }

    ///Returns the mutable timestamp when the last event was received or sent.
    pub fn get_mut_last_update(&mut self) -> Option<&mut String> {
        self.last_update.as_mut()
    }

    ///Sets the minimum interval for the maximum event frequency.
    ///
    /// [interval]: minimum interval
    pub fn set_min_interval(&mut self, interval: String) {
        self.min_interval = Some(interval);
    }

    ///Returns the minimum interval for the maximum event frequency.
    pub fn get_min_interval(&self) -> Option<&String> {
        self.min_interval.as_ref()
    }

    ///Returns the mutable minimum interval for the maximum event frequency.
    pub fn get_mut_min_interval(&mut self) -> Option<&mut String> {
        self.min_interval.as_mut()
    }

    ///Sets the maximum interval for messages about the update of the status of the event.
    ///
    /// [interval]: maximum interval
    pub fn set_max_interval(&mut self, interval: String) {
        self.max_interval = Some(interval);
    }

    ///Returns the maximum interval for messages about the update of the status of the event.
    pub fn get_max_interval(&self) -> Option<&String> {
        self.max_interval.as_ref()
    }

    ///Returns the mutable maximum interval for messages about the update of the status of the event.
    pub fn get_mut_max_interval(&mut self) -> Option<&mut String> {
        self.max_interval.as_mut()
    }
}

impl TSubmodelElement for BasicEventElement {}

impl TReferable for BasicEventElement {
    fn set_category(&mut self, category: String) {
        self.category = Some(category);
    }

    fn get_category(&self) -> Option<&String> {
        self.category.as_ref()
    }

    fn get_mut_category(&mut self) -> Option<&mut String> {
        self.category.as_mut()
    }

    fn set_id_short(&mut self, id_short: String) {
        self.id_short = Some(id_short);
    }

    fn get_id_short(&self) -> Option<&String> {
        self.id_short.as_ref()
    }

    fn get_mut_id_short(&mut self) -> Option<&mut String> {
        self.id_short.as_mut()
    }

    fn set_display_name(&mut self, display_name: Vec<MultiLanguageNameType>) {
        self.display_name = display_name;
    }

    fn get_display_name(&self) -> &Vec<MultiLanguageNameType> {
        &self.display_name
    }

    fn get_mut_display_name(&mut self) -> &mut Vec<MultiLanguageNameType> {
        &mut self.display_name
    }

    fn add_display_name(&mut self, display_name: MultiLanguageNameType) {
        self.display_name.push(display_name);
    }

    fn remove_display_name(&mut self, index: usize) -> MultiLanguageNameType {
        self.display_name.remove(index)
    }

    fn set_description(&mut self, description: Vec<MultiLanguageTextType>) {
        self.description = description;
    }

    fn get_description(&self) -> &Vec<MultiLanguageTextType> {
        &self.description
    }

    fn get_mut_description(&mut self) -> &mut Vec<MultiLanguageTextType> {
        &mut self.description
    }

    fn add_description(&mut self, description: MultiLanguageTextType) {
        self.description.push(description);
    }

    fn remove_description(&mut self, index: usize) -> MultiLanguageTextType {
        self.description.remove(index)
    }
}

impl THasExtensions for BasicEventElement {
    fn get_extensions(&self) -> &Vec<Extension> {
        &self.extensions
    }

    fn get_mut_extensions(&mut self) -> &mut Vec<Extension> {
        &mut self.extensions
    }

    fn set_extensions(&mut self, extensions: Vec<Extension>) {
        self.extensions = extensions;
    }

    fn add_extension(&mut self, extension: Extension) {
        self.extensions.push(extension);
    }

    fn remove_extension(&mut self, index: usize) -> Extension {
        self.extensions.remove(index)
    }
}

impl THasSemantics for BasicEventElement {
    fn set_semantic_id(&mut self, supplemental_semantic_id: Reference) {
        self.semantic_id = Some(supplemental_semantic_id);
    }

    fn get_semantic_id(&self) -> Option<&Reference> {
        self.semantic_id.as_ref()
    }

    fn get_mut_semantic_id(&mut self) -> Option<&mut Reference> {
        self.semantic_id.as_mut()
    }

    fn set_supplemental_semantic_ids(&mut self, supplemental_semantic_ids: Vec<Reference>) {
        self.supplemental_semantic_ids = supplemental_semantic_ids;
    }

    fn get_supplemental_semantic_ids(&self) -> &Vec<Reference> {
        &self.supplemental_semantic_ids
    }

    fn get_mut_supplemental_semantic_ids(&mut self) -> &mut Vec<Reference> {
        &mut self.supplemental_semantic_ids
    }

    fn add_supplemental_semantic_id(&mut self, semantic_id: Reference) {
        self.supplemental_semantic_ids.push(semantic_id);
    }

    fn remove_supplemental_semantic_id(&mut self, index: usize) -> Reference {
        self.supplemental_semantic_ids.remove(index)
    }
}

impl TQualifiable for BasicEventElement {
    fn set_qualifiers(&mut self, qualifiers: Vec<Qualifier>) {
        self.qualifiers = qualifiers;
    }

    fn get_qualifiers(&self) -> &Vec<Qualifier> {
        &self.qualifiers
    }

    fn get_mut_qualifiers(&mut self) -> &mut Vec<Qualifier> {
        &mut self.qualifiers
    }

    fn add_qualifier(&mut self, qualifier: Qualifier) {
        self.qualifiers.push(qualifier);
    }

    fn remove_qualifier(&mut self, index: usize) -> Qualifier {
        self.qualifiers.remove(index)
    }
}

impl THasDataSpecification for BasicEventElement {
    fn get_data_specifications(&self) -> &Vec<Reference> {
        &self.specifications
    }

    fn get_mut_data_specifications(&mut self) -> &mut Vec<Reference> {
        &mut self.specifications
    }

    fn set_data_specifications(&mut self, data_specifications: Vec<Reference>) {
        self.specifications = data_specifications;
    }

    fn add_data_specification(&mut self, data_specification: Reference) {
        self.specifications.push(data_specification);
    }

    fn remove_data_specification(&mut self, index: usize) -> Reference {
        self.specifications.remove(index)
    }
}

impl TEventElement for BasicEventElement {
    
}