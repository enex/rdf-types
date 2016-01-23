use Triple;

pub type N3Triple = Triple<N3Node>;

pub type N3Statement = N3Triple;

pub enum N3Node{
	Formula(Vec<N3Triple>),
	Exsistential(usize),
	Literal(String),
	Universal(usize)
}

