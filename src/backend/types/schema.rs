use std::fmt::Display;

pub enum VectorAlgorithm {
    Flat,
    HNSW,
}

impl Display for VectorAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            VectorAlgorithm::Flat => "FLAT".to_string(),
            VectorAlgorithm::HNSW => "HNSW".to_string(),
        };
        write!(f, "{}", str)
    }
}

pub enum VectorDataType {
    Float32,
    Float64,
}

impl Display for VectorDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            VectorDataType::Float32 => "float32".to_string(),
            VectorDataType::Float64 => "float64".to_string(),
        };
        write!(f, "{}", str)
    }   
}

pub enum DistanceMetric {
    Cosine,
    L2,
    IP,
}

impl Display for DistanceMetric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            DistanceMetric::Cosine => "cosine".to_string(),
            DistanceMetric::L2 => "l2".to_string(),
            DistanceMetric::IP => "ip".to_string(),
        };
        write!(f, "{}", str)
    }
}

pub enum FieldType {
    Vector {
        algorithm: VectorAlgorithm,
        dims: usize,
        data_type: VectorDataType,
        distance: DistanceMetric,
    },
    Text,
    Tag,
}

pub struct SchemaField {
    pub name: String,
    pub field_type: FieldType,
}

impl SchemaField {
    pub fn to_redis_args(&self) -> Vec<String> {
        match &self.field_type {
            FieldType::Vector { algorithm, dims, data_type, distance } => {
                let vector_args = vec![
                    "TYPE".into(), data_type.to_string(),
                    "DIM".into(), dims.to_string(),
                    "DISTANCE_METRIC".into(), distance.to_string(),
                ];

                let mut args = vec![
                    self.name.clone(),
                    "VECTOR".into(),
                    algorithm.to_string(),
                    vector_args.len().to_string(),
                ];

                args.extend(vector_args);
                args
            }
            FieldType::Text => vec![self.name.clone(), "TEXT".into()],
            FieldType::Tag => vec![self.name.clone(), "TAG".into()],
        }
    }
}



pub fn default_schema() -> Vec<SchemaField> {
    vec![
        SchemaField {
            name: "embedding".to_string(),
            field_type: FieldType::Vector {
                algorithm: VectorAlgorithm::HNSW,
                dims: 1536,
                data_type: VectorDataType::Float32,
                distance: DistanceMetric::Cosine,
            },
        },
        SchemaField {
            name: "text".to_string(),
            field_type: FieldType::Text,
        },
        SchemaField {
            name: "model".to_string(),
            field_type: FieldType::Tag,
        },
    ]
}
