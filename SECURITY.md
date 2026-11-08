# Security

Report a vulnerability privately through GitHub: this repository's **Security** tab, then **Report a vulnerability**.
Please don't open a public issue for one.

Skills are instructions an agent follows, so a skill that tells an agent to run something unsafe, fetch from an
untrusted place, or leak a secret counts as a vulnerability here. So does anything in `crates/skillcheck` or CI that
could run code from a pull request with more than read access.
