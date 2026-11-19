# Security policy

## Supported versions

Security fixes are made on the latest released minor version. Older versions
may be asked to upgrade before a fix is provided.

## Reporting a vulnerability

Do not open a public issue for a suspected vulnerability. Use GitHub's
**Security → Report a vulnerability** form for this repository. Include the
affected version, a minimal reproduction, impact, and any suggested remedy.

The maintainers aim to acknowledge a report within 7 days, provide an initial
assessment within 14 days, and coordinate disclosure after a fix is available.
These are response targets rather than contractual guarantees.

Please allow a reasonable remediation period before public disclosure. Good
faith research and reports that avoid privacy violations, data destruction,
and service disruption are welcome.

## Release verification

Tagged releases include SHA-256 checksums and a GitHub build-provenance
attestation for the packaged crate. Verify a downloaded crate with:

```sh
gh attestation verify software-engineering-<version>.crate \
  --repo publicprojects-io/software-engineering-rust-crate
```
