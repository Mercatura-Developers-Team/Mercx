import React from 'react';
import MyNavbar from './Navbar';
import { Route, Routes } from 'react-router-dom';
import Home from './Home';
import ActionsOptions from './Actions/ActionsOptions';
import { AuthProvider } from "./use-auth-client";
import Transcations from './Transcations';
import Transfer from './Transfer';
import SignupForm from './SignupForm';
import InappBrowser from './InappBrowser';
function App() {
  return (
    <AuthProvider>
      <div className="overflow-hidden">

        <MyNavbar />
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/trade" element={<ActionsOptions />} />
          <Route path="/transactions" element={<Transcations />} />
          <Route path="/wallet" element={<Transfer />} />
          <Route path="/signup" element={<SignupForm />} />
          <Route path="/InappBrowser" element={<InappBrowser/>} />
        </Routes>
      </div>
    </AuthProvider>
  );
}

export default App;
