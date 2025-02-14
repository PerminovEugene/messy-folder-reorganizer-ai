#!/bin/bash
echo "Installing Git hooks..."
cp hooks/pre-commit .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
echo "Pre-commit hook installed successfully!"
