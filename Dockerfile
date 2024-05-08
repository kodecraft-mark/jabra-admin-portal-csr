# Stage 1: Build the application
FROM rust:slim AS builder

WORKDIR /app

# Copy only the necessary files for building
COPY . .

# Install dependencies and build the application with trunk
RUN cargo install trunk@0.19.3
RUN rustup toolchain install nightly
RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown
RUN trunk build --release

# Stage 2: Create a lightweight image to run the app
FROM rust:slim

WORKDIR /app

# Copy only the necessary files from the builder stage
COPY --from=builder /app/dist ./dist
COPY --from=builder /app/index.html .  
COPY --from=builder /app/style ./style
COPY --from=builder /app/public ./public
COPY --from=builder /app/Cargo.toml /app/
COPY --from=builder /app/src ./src
COPY --from=builder /app/Cargo.lock /app/
COPY --from=builder /app/index.html /app/
COPY --from=builder /app/input.css /app/
COPY --from=builder /app/package-lock.json /app/
COPY --from=builder /app/package.json /app/
COPY --from=builder /app/tailwind.config.js /app/
COPY --from=builder /app/rust-toolchain.toml /app/

# Install trunk in the runner stage
RUN cargo install trunk@0.19.3 \
    && rustup target add wasm32-unknown-unknown

# RUN npx tailwindcss -i ./input.css -o ./style/output.css
# RUN npm install -D daisyui@latest

# Expose port 8080
EXPOSE 8080

# Start your app
CMD ["trunk", "serve", "--release", "-v"]