#!/bin/bash
# Run GitHub Actions locally using act
# Requires: https://github.com/nektos/act

# Run the Test workflow on all platforms
act -j Test --container-architecture linux/amd64
