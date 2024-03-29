pub enum RegexRepeticiones {
    Simple,
    Exacto(usize),
    Rango{
        min: Option<usize>,
        max: Option<usize>
    }
}