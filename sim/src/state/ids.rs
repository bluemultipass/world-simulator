use serde::{Deserialize, Serialize};

macro_rules! id_newtype {
    ($name:ident) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            Serialize,
            Deserialize,
            Default,
        )]
        pub struct $name(pub u64);
    };
}

id_newtype!(AgentId);
id_newtype!(CohortId);
id_newtype!(CivId);
id_newtype!(ConceptId);
id_newtype!(CapabilityId);
id_newtype!(TileId);
id_newtype!(RegionId);

/// Alias used in Agent.knowledge to key per-capability mastery levels.
pub type Domain = CapabilityId;
