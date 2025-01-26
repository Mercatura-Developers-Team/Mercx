import React from 'react';
import MyNavbar from './Navbar';
import { Route, Routes } from 'react-router-dom';
import Home from './Home';
import ActionsOptions from './Actions/ActionsOptions';
import {  AuthProvider } from "./use-auth-client";
import Transcations from './Transcations';
import Transfer from './Transfer';

function App() {
  return (
    <AuthProvider>
    <div>
      <MyNavbar />
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/trade" element={<ActionsOptions />} />
        <Route path="/transactions" element={ <Transcations />} />
        <Route path="/transfer"  element={ <Transfer/>} />
      </Routes>
    </div>
    </AuthProvider>
  );
}

export default App;
