# Base image for the ICP Dev Environment
FROM ghcr.io/dfinity/icp-dev-env:latest

# Set working directory inside the container
WORKDIR /workspace

# Install Node.js (if not already included in the base image)
RUN apt-get update && apt-get install -y \
    curl \
    && curl -fsSL https://deb.nodesource.com/setup_16.x | bash - \
    && apt-get install -y nodejs

# Install any additional build tools required for Tailwind (if needed)
RUN apt-get install -y build-essential

# Clean up apt cache to reduce image size
RUN apt-get clean

# Install Tailwind CLI globally (optional, only if you need the CLI directly)
RUN npm install -g tailwindcss
