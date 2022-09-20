FROM rustlang/rust:nightly

ADD . rustc-llvm-error/
WORKDIR rustc-llvm-error/

RUN rustc --version --verbose

# check succeeds
RUN cargo check
# but bulid doesn't
ENTRYPOINT cargo build
