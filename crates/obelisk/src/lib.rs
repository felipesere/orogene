use cxx;

#[cxx::bridge(namespace = "node")]
mod ffi {

    extern "C" {
        include!("node.h");

        fn InitializeNodeWithArgs(
            args: *const CxxVector<CxxString>,
            exec_args: *const CxxVector<CxxString>,
            errors: *const CxxVector<CxxString>,
        ) -> i32;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let args = Vec::new();
        let exec_args = Vec::new();
        let errors = Vec::new();

        ffi::InitializeNodeWithArgs(args, exec_args, errors);
    }
}
