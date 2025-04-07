use std::any::TypeId;
use std::hash;
use std::hash::Hasher;

pub trait BoxedBuildHasher {
    fn clone_boxed(&self) -> Box<dyn BoxedBuildHasher>;
    fn build_hasher_boxed(&self) -> Box<dyn Hasher>;
}

impl<S> BoxedBuildHasher for S
where
    S: hash::BuildHasher + Clone + 'static,
{
    fn clone_boxed(&self) -> Box<dyn BoxedBuildHasher> {
        Box::new(self.clone())
    }

    fn build_hasher_boxed(&self) -> Box<dyn Hasher> {
        Box::new(self.build_hasher())
    }
}

pub struct OpaqueHasher {
    hasher: Box<dyn hash::Hasher>,
    type_id: TypeId,
}

impl OpaqueHasher {
    #[inline]
    const fn new(hasher: Box<dyn hash::Hasher>, type_id: TypeId) -> Self {
        Self {
            hasher,
            type_id,
        }
    }

    pub fn is_hasher_type<H>(&self) -> bool
    where
        H: hash::Hasher + 'static,
    {
        self.type_id == TypeId::of::<H>()
    }
}

impl hash::Hasher for OpaqueHasher {
    fn finish(&self) -> u64 {
        self.hasher.finish()
    }

    fn write(&mut self, bytes: &[u8]) {
        self.hasher.write(bytes)
    }
}

pub struct OpaqueBuildHasher {
    build_hasher: Box<dyn BoxedBuildHasher>,
    builder_type_id: TypeId,
    hasher_type_id: TypeId,
}

impl OpaqueBuildHasher {
    #[inline]
    pub fn new<S>(build_hasher: Box<S>) -> Self
    where
        S: hash::BuildHasher + Clone + 'static,
    {
        let builder_type_id: TypeId = TypeId::of::<S>();
        let hasher_type_id = TypeId::of::<S::Hasher>();

        Self {
            build_hasher,
            builder_type_id,
            hasher_type_id,
        }
    }

    #[inline]
    pub fn is_builder_type<S>(&self) -> bool
    where
        S: hash::BuildHasher + Clone + 'static,
    {
        self.builder_type_id == TypeId::of::<S>()
    }

    #[inline]
    pub fn is_hasher_type<H>(&self) -> bool
    where
        H: hash::Hasher + 'static,
    {
        self.hasher_type_id == TypeId::of::<H>()
    }
}

impl Clone for OpaqueBuildHasher {
    fn clone(&self) -> Self {
        Self {
            build_hasher: self.build_hasher.clone_boxed(),
            builder_type_id: self.builder_type_id,
            hasher_type_id: self.hasher_type_id,
        }
    }
}

impl hash::BuildHasher for OpaqueBuildHasher {
    type Hasher = OpaqueHasher;

    fn build_hasher(&self) -> Self::Hasher {
        let hasher= self.build_hasher.build_hasher_boxed();
        let type_id = self.hasher_type_id;

        OpaqueHasher::new(hasher, type_id)
    }
}
