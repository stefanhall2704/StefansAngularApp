## Getting Started

To get started with this project, please follow the steps below:

### Prerequisites

- Ensure you have [Minikube](https://minikube.sigs.k8s.io/docs/start/) installed on your system. Minikube is required to run this project with Kubernetes.

### Initial Setup

1. **Set Up Executable Permissions**
    ```sh
    chmod +x kubernetes/deploy.sh kubernetes/restart.sh
    ```

1. **Running with Minikube**
This project is designed to run on a Kubernetes cluster managed by Minikube. Please ensure you have Minikube installed and properly configured. You can find the installation and setup instructions in the [Minikube](https://minikube.sigs.k8s.io/docs/).

2. **Start Minikube**

   Start your Minikube cluster with the following command:

   ```sh
   minikube start
   ```

### Deploy the Application

Deploy the application to your Minikube cluster using the deployment script:

```sh
cd kubernetes
./deploy.sh
```

### Access the Application
Once the deployment is complete, you can access the application using the NodePort service. Use the following command to get the URL:

```sh
minikube service --all
```

### Restart the Application
If you need to restart the application, use the restart script:
```sh
cd kubernetes
./restart.sh
```
