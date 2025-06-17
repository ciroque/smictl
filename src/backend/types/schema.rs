use std::fmt::Display;
use std::str::FromStr;
use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "lower")]
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

#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "lower")]
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

#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "lower")]
pub enum DistanceMetric {
    Cosine,
    L2,
    IP,
}

impl Display for DistanceMetric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            DistanceMetric::Cosine => "COSINE".to_string(),
            DistanceMetric::L2 => "L2".to_string(),
            DistanceMetric::IP => "IP".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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



// pub fn default_schema() -> Vec<SchemaField> {
//     vec![
//         SchemaField {
//             name: "embedding".to_string(),
//             field_type: FieldType::Vector {
//                 algorithm: VectorAlgorithm::HNSW,
//                 dims: 1536,
//                 data_type: VectorDataType::Float32,
//                 distance: DistanceMetric::Cosine,
//             },
//         },
//         SchemaField {
//             name: "text".to_string(),
//             field_type: FieldType::Text,
//         },
//         SchemaField {
//             name: "model".to_string(),
//             field_type: FieldType::Tag,
//         },
//     ]
// }

#[derive(Debug, Clone)]
pub struct SchemaArg(pub Vec<SchemaField>);

impl FromStr for SchemaArg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields = s
            .split(',')
            .map(|part| {
                let mut parts = part.splitn(2, ':');
                let name = parts
                    .next()
                    .ok_or("Missing field name")?
                    .to_string();
                let type_str = parts
                    .next()
                    .ok_or("Missing field type")?
                    .to_lowercase();

                let field_type = match type_str.as_str() {
                    "text" => FieldType::Text,
                    "tag" => FieldType::Tag,
                    "vector" => {
                        // Placeholder values for now; user must override dim/type/distance elsewhere
                        FieldType::Vector {
                            algorithm: VectorAlgorithm::HNSW,
                            dims: 0,
                            data_type: VectorDataType::Float32,
                            distance: DistanceMetric::Cosine,
                        }
                    }
                    other => return Err(format!("Unknown field type: {}", other)),
                };

                Ok(SchemaField { name, field_type })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(SchemaArg(fields))
    }
}

impl SchemaArg {
    pub fn apply_vector_params(
        &mut self,
        algorithm: Option<VectorAlgorithm>,
        distance: Option<DistanceMetric>,
        dtype: Option<VectorDataType>,
        dim: Option<usize>,
    ) {
        for field in &mut self.0 {
            if let FieldType::Vector {
                algorithm: a,
                distance: d,
                data_type: dt,
                dims
            } = &mut field.field_type {
                if let Some(new_a) = algorithm.clone() {
                    *a = new_a;
                }
                if let Some(new_d) = distance.clone() {
                    *d = new_d;
                }
                if let Some(new_dt) = dtype.clone() {
                    *dt = new_dt;
                }
                if let Some(new_dims) = dim {
                    *dims = new_dims;
                }
            }
        }
    }
}
