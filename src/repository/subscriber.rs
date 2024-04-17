use dashmap::Dashmap;
use lazy_static::lazy_static;
use create::model::subscriber::Subscriber;

// Singleton of database
lazy_static! {
    static ref SUBSCRIBER: Dashmap<String, Dashmap<String, Subscriber>> = Dashmap::new();
}

pub struct SubscriberRepository;

impl SubscriberRepository {
}