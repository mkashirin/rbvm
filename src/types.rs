pub struct BoundedUsize<const L: usize, const H: usize>(usize);

impl<const L: usize, const H: usize> BoundedUsize<{ L }, { H }> {
    pub const LOW: usize = L;
    pub const HIGH: usize = H;

    pub fn new(value: usize) -> Self {
        BoundedUsize(value.min(Self::HIGH).max(Self::LOW))
    }

    pub fn fallible_new(value: usize) -> Result<Self, &'static str> {
        match value {
            value if value < Self::LOW => Err("Value too low"),
            value if value > Self::HIGH => Err("Value too high"),
            value => Ok(BoundedUsize(value)),
        }
    }

    pub fn set(&mut self, value: usize) {
        *self = BoundedUsize(value.min(Self::HIGH).max(Self::LOW))
    }
}

impl<const L: usize, const H: usize> std::ops::Deref
    for BoundedUsize<{ L }, { H }>
{
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
