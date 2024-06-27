
# examplepw

A simple tutorial on how to install an IOTA node with IOTA Sandbox on your Linux device and interact with it.

## Prerequisites

To get started, ensure you have a Linux operating system and the following dependencies installed:

- A recent release of Docker
- Docker Compose CLI plugin
- `sed`
- `jq`

These components are essential for setting up the IOTA Sandbox environment on a Linux system, ensuring all necessary tools are in place for proper operation.

## Installation

### Step 1: Download the Latest Version

Run the following commands to download the latest version of IOTA Sandbox:

```bash
mkdir iota-sandbox
cd iota-sandbox
curl -L https://github.com/iotaledger/iota-sandbox/releases/latest/download/iota_sandbox.tar.gz | tar -zx
```

### Step 2: Bootstrap the Sandbox

To bootstrap the IOTA Sandbox, run:

\`\`\`bash
sudo ./bootstrap.sh
\`\`\`

### Step 3: Start the IOTA Sandbox

Finally, start the IOTA Sandbox with:

\`\`\`bash
docker compose up -d
\`\`\`

## Examples

You can find three main examples demonstrating different functionalities with IOTA:

1. [Create a DID with IOTA](https://github.com/Allevs01/examplepw/blob/main/examplepw/src/createdid.rs)
2. [Issue a Credential with IOTA](https://github.com/Allevs01/examplepw/blob/main/examplepw/src/issuevc.rs)
3. [Push and Retrieve a Block from the IOTA Tangle](https://github.com/Allevs01/examplepw/blob/main/examplepw/src/pushandretrieve.rs), implemented together with DIDs and VCs

These examples provide practical insights into interacting with the IOTA network using DIDs and VCs.
