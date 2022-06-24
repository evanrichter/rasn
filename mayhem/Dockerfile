# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest as builder

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN echo building instrumented harnesses && \
    cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/cms_signeddata /cms_signeddata && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/kerberos_enckdcreppart /kerberos_enckdcreppart && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/kerberos_encticketpart /kerberos_encticketpart && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/kerberos_kdcrep /kerberos_kdcrep && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/kerberos_kdcreq /kerberos_kdcreq && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/kerberos_ticket /kerberos_ticket && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/ldap_message /ldap_message && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/ldap_result /ldap_result && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/ocsp_request /ocsp_request && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/ocsp_response /ocsp_response && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/pkix /pkix && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/smime_encpreference /smime_encpreference && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/snmpv1_getrequest /snmpv1_getrequest && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/snmpv2c_message /snmpv2c_message && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/snmpv2_getbulkrequest /snmpv2_getbulkrequest && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/snmpv3_message /snmpv3_message && \
    echo done

RUN echo building non-instrumented harnesses && \
    cargo +nightly -Z sparse-registry fuzz build --fuzz-dir fuzzing/cargo-fuzz --disable-instrumentation && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/cms_signeddata /cms_signeddata_no_inst && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/kerberos_enckdcreppart /kerberos_enckdcreppart_no_inst && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/kerberos_encticketpart /kerberos_encticketpart_no_inst && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/kerberos_kdcrep /kerberos_kdcrep_no_inst && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/kerberos_kdcreq /kerberos_kdcreq_no_inst && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/kerberos_ticket /kerberos_ticket_no_inst && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/ldap_message /ldap_message_no_inst && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/ldap_result /ldap_result_no_inst && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/ocsp_request /ocsp_request_no_inst && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/ocsp_response /ocsp_response_no_inst && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/pkix /pkix_no_inst && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/smime_encpreference /smime_encpreference_no_inst && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/snmpv1_getrequest /snmpv1_getrequest_no_inst && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/snmpv2c_message /snmpv2c_message_no_inst && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/snmpv2_getbulkrequest /snmpv2_getbulkrequest_no_inst && \
    mv fuzzing/cargo-fuzz/target/x86_64-unknown-linux-gnu/release/snmpv3_message /snmpv3_message_no_inst && \
    echo done

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /cms_signeddata  /cms_signeddata_no_inst /
COPY --from=builder /kerberos_enckdcreppart  /kerberos_enckdcreppart_no_inst /
COPY --from=builder /kerberos_encticketpart  /kerberos_encticketpart_no_inst /
COPY --from=builder /kerberos_kdcrep  /kerberos_kdcrep_no_inst /
COPY --from=builder /kerberos_kdcreq  /kerberos_kdcreq_no_inst /
COPY --from=builder /kerberos_ticket  /kerberos_ticket_no_inst /
COPY --from=builder /ldap_message  /ldap_message_no_inst /
COPY --from=builder /ldap_result  /ldap_result_no_inst /
COPY --from=builder /ocsp_request  /ocsp_request_no_inst /
COPY --from=builder /ocsp_response  /ocsp_response_no_inst /
COPY --from=builder /pkix  /pkix_no_inst /
COPY --from=builder /smime_encpreference  /smime_encpreference_no_inst /
COPY --from=builder /snmpv1_getrequest  /snmpv1_getrequest_no_inst /
COPY --from=builder /snmpv2c_message  /snmpv2c_message_no_inst /
COPY --from=builder /snmpv2_getbulkrequest  /snmpv2_getbulkrequest_no_inst /
COPY --from=builder /snmpv3_message  /snmpv3_message_no_inst /
