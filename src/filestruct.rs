#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct FileName {
    pub pathfile: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Fasta {
    pub idname: String,
    pub nameseq: String,
}
