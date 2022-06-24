# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest as builder

## Add source code to the build stage.
ADD . /src
WORKDIR /src

## Build cms_signeddata
RUN cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz -- cms_signeddata && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/cms_signeddata /cms_signeddata && \
    cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz --disable-instrumentation -- cms_signeddata && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/cms_signeddata /cms_signeddata_no_inst
## Build kerberos_enckdcreppart
RUN cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz -- kerberos_enckdcreppart && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/kerberos_enckdcreppart /kerberos_enckdcreppart && \
    cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz --disable-instrumentation -- kerberos_enckdcreppart && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/kerberos_enckdcreppart /kerberos_enckdcreppart_no_inst
## Build kerberos_encticketpart
RUN cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz -- kerberos_encticketpart && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/kerberos_encticketpart /kerberos_encticketpart && \
    cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz --disable-instrumentation -- kerberos_encticketpart && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/kerberos_encticketpart /kerberos_encticketpart_no_inst
## Build kerberos_kdcrep
RUN cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz -- kerberos_kdcrep && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/kerberos_kdcrep /kerberos_kdcrep && \
    cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz --disable-instrumentation -- kerberos_kdcrep && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/kerberos_kdcrep /kerberos_kdcrep_no_inst
## Build kerberos_kdcreq
RUN cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz -- kerberos_kdcreq && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/kerberos_kdcreq /kerberos_kdcreq && \
    cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz --disable-instrumentation -- kerberos_kdcreq && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/kerberos_kdcreq /kerberos_kdcreq_no_inst
## Build kerberos_ticket
RUN cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz -- kerberos_ticket && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/kerberos_ticket /kerberos_ticket && \
    cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz --disable-instrumentation -- kerberos_ticket && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/kerberos_ticket /kerberos_ticket_no_inst
## Build ldap_message
RUN cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz -- ldap_message && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/ldap_message /ldap_message && \
    cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz --disable-instrumentation -- ldap_message && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/ldap_message /ldap_message_no_inst
## Build ldap_result
RUN cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz -- ldap_result && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/ldap_result /ldap_result && \
    cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz --disable-instrumentation -- ldap_result && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/ldap_result /ldap_result_no_inst
## Build ocsp_request
RUN cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz -- ocsp_request && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/ocsp_request /ocsp_request && \
    cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz --disable-instrumentation -- ocsp_request && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/ocsp_request /ocsp_request_no_inst
## Build ocsp_response
RUN cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz -- ocsp_response && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/ocsp_response /ocsp_response && \
    cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz --disable-instrumentation -- ocsp_response && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/ocsp_response /ocsp_response_no_inst
## Build pkix
RUN cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz -- pkix && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/pkix /pkix && \
    cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz --disable-instrumentation -- pkix && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/pkix /pkix_no_inst
## Build smime_encpreference
RUN cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz -- smime_encpreference && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/smime_encpreference /smime_encpreference && \
    cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz --disable-instrumentation -- smime_encpreference && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/smime_encpreference /smime_encpreference_no_inst
## Build snmpv1_getrequest
RUN cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz -- snmpv1_getrequest && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/snmpv1_getrequest /snmpv1_getrequest && \
    cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz --disable-instrumentation -- snmpv1_getrequest && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/snmpv1_getrequest /snmpv1_getrequest_no_inst
## Build snmpv2c_message
RUN cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz -- snmpv2c_message && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/snmpv2c_message /snmpv2c_message && \
    cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz --disable-instrumentation -- snmpv2c_message && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/snmpv2c_message /snmpv2c_message_no_inst
## Build snmpv2_getbulkrequest
RUN cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz -- snmpv2_getbulkrequest && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/snmpv2_getbulkrequest /snmpv2_getbulkrequest && \
    cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz --disable-instrumentation -- snmpv2_getbulkrequest && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/snmpv2_getbulkrequest /snmpv2_getbulkrequest_no_inst
## Build snmpv3_message
RUN cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz -- snmpv3_message && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/snmpv3_message /snmpv3_message && \
    cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz --disable-instrumentation -- snmpv3_message && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/snmpv3_message /snmpv3_message_no_inst

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /cms_signeddata /
COPY --from=builder /cms_signeddata_no_inst /
COPY --from=builder /kerberos_enckdcreppart /
COPY --from=builder /kerberos_enckdcreppart_no_inst /
COPY --from=builder /kerberos_encticketpart /
COPY --from=builder /kerberos_encticketpart_no_inst /
COPY --from=builder /kerberos_kdcrep /
COPY --from=builder /kerberos_kdcrep_no_inst /
COPY --from=builder /kerberos_kdcreq /
COPY --from=builder /kerberos_kdcreq_no_inst /
COPY --from=builder /kerberos_ticket /
COPY --from=builder /kerberos_ticket_no_inst /
COPY --from=builder /ldap_message /
COPY --from=builder /ldap_message_no_inst /
COPY --from=builder /ldap_result /
COPY --from=builder /ldap_result_no_inst /
COPY --from=builder /ocsp_request /
COPY --from=builder /ocsp_request_no_inst /
COPY --from=builder /ocsp_response /
COPY --from=builder /ocsp_response_no_inst /
COPY --from=builder /pkix /
COPY --from=builder /pkix_no_inst /
COPY --from=builder /smime_encpreference /
COPY --from=builder /smime_encpreference_no_inst /
COPY --from=builder /snmpv1_getrequest /
COPY --from=builder /snmpv1_getrequest_no_inst /
COPY --from=builder /snmpv2c_message /
COPY --from=builder /snmpv2c_message_no_inst /
COPY --from=builder /snmpv2_getbulkrequest /
COPY --from=builder /snmpv2_getbulkrequest_no_inst /
COPY --from=builder /snmpv3_message /
COPY --from=builder /snmpv3_message_no_inst /
