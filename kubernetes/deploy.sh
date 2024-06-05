#!/bin/bash

kubectl apply -f housing-frontend.yaml
kubectl apply -f housing-backend.yaml
kubectl apply -f ingress.yaml
kubectl apply -f ingress-controller.yaml
