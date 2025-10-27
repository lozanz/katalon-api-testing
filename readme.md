# AfterOffice Academy QA
API Testing project using Katalon Studio

Project Includes:
- Test Objects (GET All, GET Single, POST, PATCH, DELETE)
- Test Cases (each for one endpoint) 
- Chained Test Case (CRUD flow) 
- Test Suite & Test Suite Collection 
- Validation of status code and JSON schema 
- Environment variables via Profile (Base URL, API Key & Authorization) 

Example Endpoints :
```
GET    /users
GET    /users?username=eq.broot
POST   /users
PATCH  /users?username=eq.broot
DELETE /users?username=eq.broot
```

How to run test : 
Set env-Dev Profile
```
Test Suites > TS API Users
Test Suites > TS Workflow User CRUD
Test Suite Collection > Mock API Users
```

