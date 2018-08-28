/******************************************************************************
 *                                            TRAITS
 ******************************************************************************/

pub trait AsBytes<T=Self> {

    fn to_bytes(&self, target: &mut Vec<u8>) -> Result<(), &'static str>;

    fn from_bytes(bytes: &[u8]) -> Result<T, &'static str>;

}

/*----------------------------------------------------------------------------*/

pub trait AsStr<T=Self> {

    fn from_str(string : &str) -> Result<T, ()>;

}

/*----------------------------------------------------------------------------*/

pub trait DnsEntity<T=Self> : AsBytes<T> + AsStr<T> {}
