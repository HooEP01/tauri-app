export const environment = {
    production: false,
    name: 'Tauri Angular',
    api: {
      // baseUrl: '/api',
      baseUrl: 'https://jsonplaceholder.typicode.com',
      // baseUrl: 'https://hustle-api.kulaijaya.com/',
    },
    web: {
      baseUrl: 'http://localhost:1420'
    },
    firebase: {
      config: {
        apiKey: 'AIzaSyBFmG2X2tNNMlbIavrl7tpgoPLWJUB5Xu4',
        authDomain: 'hustle-hero-uat.firebaseapp.com',
        projectId: 'hustle-hero-uat',
        storageBucket: 'hustle-hero-uat.appspot.com',
        messagingSenderId: '175781881485',
        appId: '1:175781881485:web:503a1fafe57a0cbf34e4c4',
        measurementId: 'G-G099RN4SBY'
      },
    }
  };
  