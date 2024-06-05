#!/bin/bash

kubectl rollout restart deployment frontend-depl
kubectl rollout restart deployment backend-depl
# kubectl rollout restart deployment nginx-depl
