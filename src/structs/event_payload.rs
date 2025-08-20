use crate::structs::reference::Reference;

///Defines the necessary information of an event instance sent out or received.
#[derive(PartialEq, Clone)]
pub struct EventPayload {
    ///Reference to the source event element.
    source: Reference,
    ///Optional semantic ID of the source event element if available.
    source_semantic_id: Option<Reference>,
    ///Reference to the referable which defines the scope of the event.
    observable_reference: Reference,
    ///Optional semantic ID of the referable which defines the scope of the event if available.
    observable_semantic_id: Option<Reference>,
    ///Optional information for the outer message infrastructure to schedule the event for the respective
    ///communication channel.
    topic: Option<String>,
    ///Optional subject who/which initiated the creation.
    subject_id: Option<Reference>,
    ///Timestamp in UTC when this event was triggered.
    time_stamp: String,
    ///Optional event-specific payload.
    payload: Vec<u8>
}

impl EventPayload {
    ///Creates a new `EventPayload`.
    ///
    /// [source]: reference to the source event element
    /// [observable_reference]: reference to the referable which defines the scope of the event
    /// [time_stamp]: timestamp in UTC when this event was triggered
    pub fn new(source: Reference, observable_reference: Reference, time_stamp: String) -> EventPayload {
        EventPayload {
            source,
            source_semantic_id: None,
            observable_reference,
            observable_semantic_id: None,
            topic: None,
            subject_id: None,
            time_stamp,
            payload: Vec::new()
        }
    }

    ///Sets the reference to the source event element.
    ///
    /// [source]: reference to the source event element
    pub fn set_source(&mut self, source: Reference){
        self.source = source;
    }

    ///Returns the reference to the source event element.
    pub fn get_source(&self) -> &Reference {
        &self.source
    }

    ///Sets the optional semantic ID of the source event element.
    ///
    /// [source_semantic_id]: semantic ID of the source event element
    pub fn set_source_semantic_id(&mut self, source_semantic_id: Reference) {
        self.source_semantic_id = Some(source_semantic_id);
    }

    ///Returns the optional semantic ID of the source event element.
    pub fn get_source_semantic_id(&self) -> Option<&Reference> {
        self.source_semantic_id.as_ref()
    }

    ///Sets the reference to the referable which defines the scope of the event.
    ///
    /// [observable_reference]: reference to the referable which defines the scope of the event
    pub fn set_observable_reference(&mut self, observable_reference: Reference) {
        self.observable_reference = observable_reference;
    }

    ///Returns the reference to the referable which defines the scope of the event.
    pub fn get_observable_reference(&self) -> &Reference {
        &self.observable_reference
    }

    ///Sets the optional semantic ID of the referable which defines the scope of the event.
    ///
    /// [observable_semantic_id]: semantic ID of the referable which defines the scope of the event
    pub fn set_observable_semantic_id(&mut self, observable_semantic_id: Reference) {
        self.observable_semantic_id = Some(observable_semantic_id);
    }

    ///Returns the optional semantic ID of the referable which defines the scope of the event.
    pub fn get_observable_semantic_id(&self) -> Option<&Reference> {
        self.observable_semantic_id.as_ref()
    }

    ///Sets the optional information for the outer message infrastructure to schedule the event for
    /// the respective communication channel.
    ///
    /// [topic]: information for the outer message infrastructure to schedule the event
    pub fn set_topic(&mut self, topic: String) {
        self.topic = Some(topic);
    }

    ///Returns the optional information for the outer message infrastructure to schedule the event
    /// for the respective communication channel.
    pub fn get_topic(&self) -> Option<&String> {
        self.topic.as_ref()
    }

    ///Sets the optional subject who/which initiated the creation.
    ///
    /// [subject_id]: subject who/which initiated the creation
    pub fn set_subject_id(&mut self, subject_id: Reference) {
        self.subject_id = Some(subject_id);
    }

    ///Returns the optional subject who/which initiated the creation.
    pub fn get_subject_id(&self) -> Option<&Reference> {
        self.subject_id.as_ref()
    }

    ///Sets the timestamp in UTC when this event was triggered.
    ///
    /// [time_stamp]: timestamp in UTC when this event was triggered
    pub fn set_time_stamp(&mut self, time_stamp: String) {
        self.time_stamp = time_stamp;
    }

    ///Returns the timestamp in UTC when this event was triggered.
    pub fn get_time_stamp(&self) -> &String {
        &self.time_stamp
    }

    ///Sets the optional event-specific payload.
    ///
    /// [payload]: the event-specific payload
    pub fn set_payload(&mut self, payload: Vec<u8>){
        self.payload = payload;
    }

    ///Returns the optional event-specific payload.
    pub fn get_payload(&self) -> &Vec<u8> {
        &self.payload
    }
}