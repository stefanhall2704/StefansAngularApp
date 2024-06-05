# Stage 1: Build the frontend application
FROM node:latest as build
WORKDIR /app
COPY package*.json /app/
RUN npm install
COPY . /app
RUN npm run build -- --output-path=./dist/out

# Stage 2: Serve the frontend application with Nginx and set up reverse proxy
FROM nginx:alpine
COPY --from=build /app/dist/out/ /usr/share/nginx/html

# Copy the Nginx configuration file
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]

