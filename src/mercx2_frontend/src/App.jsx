import React from 'react';
import MyNavbar from './Navbar';
import { Route, Routes, BrowserRouter } from 'react-router-dom';  // Import BrowserRouter
// import Home from './Home';
// import ActionsOptions from './Actions/ActionsOptions';
// import Transactions from './Transactions';
// import Transfer from './Transfer';
import { AuthProvider } from './use-auth-client';

function App() {
  return (
    <AuthProvider>
      <BrowserRouter> {/* Add BrowserRouter to wrap your app */}
        <div>
          <MyNavbar />
          <Routes>
            {/* Uncomment and add your routes here */}
            {/* <Route path="/" element={<Home />} 
            <Route path="/trade" element={<ActionsOptions />} />
            <Route path="/transactions" element={<Transactions />} />
            <Route path="/transfer" element={<Transfer />} /> */}
          </Routes>
        </div>
      </BrowserRouter>  {/* Close BrowserRouter */}
    </AuthProvider>
  );
}

export default App;
