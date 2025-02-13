import React, { useEffect, useState } from "react";
import { useAuth } from "./use-auth-client";
import { useFormik } from "formik";
import * as Yup from "yup";


// Validation schema
const SignupSchema = Yup.object().shape({
  username: Yup.string()
    .min(6, "Username must be at least 6 characters")
    .max(20, "Username must be at most 20 characters")
    .matches(/^[a-zA-Z0-9_-]+$/, "Username can only contain letters, numbers, underscores, and hyphens")
    .required("Username is required"),
    fullname: Yup.string()
    .min(2, "Fullname must be at least 2 characters")
    .max(50, "Fullname must be at most 50 characters")
    .required("Fullname is required"),

  email: Yup.string()
    .email("Invalid email address") 
    .required("Email is required"),

  phone: Yup.string()
    .matches(
      /^(\+?\d{1,4}[\s-]?)?(\(?\d{2,4}\)?[\s-]?)?\d{6,10}$/,
      "Invalid phone number"
    ) 
    .required("Phone number is required"),

  });

const SignupForm = () => {
  const { isAuthenticated, kycActor } = useAuth();
  const [kycStatus, setKycStatus] = useState(false);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");
  const { principal } = useAuth();

  // Check KYC Status
  useEffect(() => {
    if (isAuthenticated) {
      (async () => {
        try {
          const status = await kycActor.check_kyc_status(principal);
          console.log(status);
          setKycStatus(status);
        } catch (err) {
          console.error("Error checking KYC:", err);
        }
      })();
    }
  }, [isAuthenticated]);

  const formik = useFormik({
    initialValues: {
      username: "",
      fullname: "",
      phone: "",
      email: "",
    },
    validationSchema: SignupSchema,
    onSubmit: async (values) => {
      setLoading(true);
      setError("");

      try {
        const response = await kycActor.signup({
          username: values.username,
          full_name: values.fullname,
          phone_number: values.phone,
          email: values.phone,
        });

        if (response && response.Ok) {
          alert("Signup successful!");
        } else {
          throw new Error(response.Err || "Signup failed");
        }
      } catch (err) {
        // console.error("Signup Error:", err);
        setError(err.message || "Signup failed. Please try again."); // ✅ Show error message
      } finally {
        setLoading(false);
      }
    },
  });

  if (!isAuthenticated) {
    return <p>Please connect your wallet to sign up.</p>;
  }
  return (<>
    <div className="max-w-md mx-auto bg-gray-900 p-6 rounded shadow-md m-20">
      <h2 className="text-2xl font-bold text-white mb-4">Signup</h2>

      {error && <p className="text-red-500">{error}</p>}

      <form onSubmit={formik.handleSubmit} className="max-w-sm mx-auto">
        <div className="relative z-0 w-full mb-5 group" >

          <input
            id="username"
            name="username"
            type="text"
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
            value={formik.values.username}
            className="block py-2.5 px-0 w-full text-sm  bg-transparent border-0 border-b-2  appearance-none text-white border-gray-600 focus:border-blue-500 focus:outline-none focus:ring-0  peer"
          />
          <label htmlFor="username" className="peer-focus:font-medium absolute text-sm  text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 rtl:peer-focus:translate-x-1/4 peer-focus:text-blue-500  peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6">Username</label>
          {formik.touched.username && formik.errors.username && (
            <p className="text-red-400 text-sm">{formik.errors.username}</p>
          )}
          </div>
          <div class="relative z-0 w-full mb-5 group">

  
          <label htmlFor="fullname" className="peer-focus:font-medium absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 rtl:peer-focus:translate-x-1/4 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6"
  >FullName</label>
          <input
            id="fullname"
            name="fullname"
            type="text"
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
            value={formik.values.fullname}
            className="block py-2.5 px-0 w-full text-sm text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer"
          />
          {formik.touched.fullname && formik.errors.fullname && (
            <p className="text-red-400 text-sm">{formik.errors.fullname}</p>
          )}
           
        </div>
        <div className="relative z-0 w-full mb-5 group">
          <input type="tel"  name="phone" id="phone" className="block py-2.5 px-0 w-full text-sm text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer" placeholder=" " onChange={formik.handleChange}
            onBlur={formik.handleBlur}
            value={formik.values.phone} />
          <label htmlFor="phone" className="peer-focus:font-medium absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 rtl:peer-focus:translate-x-1/4 peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6">Phone number </label>
          {formik.touched.phone && formik.errors.phone && (
            <p className="text-red-400 text-sm">{formik.errors.phone}</p>
          )}
        </div>
        <div className="relative z-0 w-full mb-5 group">
          <input type="email" name="email" id="email" 
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
            value={formik.values.email}
          className="block py-2.5 px-0 w-full text-sm text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer" placeholder=" " required />
          <label htmlFor="email" className="peer-focus:font-medium absolute text-sm text-gray-500 dark:text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6">Email address</label>
          {formik.touched.email && formik.errors.email && (
            <p className="text-red-400 text-sm">{formik.errors.email}</p>
          )}
        </div>
        <div className="flex items-start mb-5">
          <div className="flex items-center h-5">
            <input id="remember" type="checkbox" value="" className="w-4 h-4 border  rounded-sm  focus:ring-3  bg-gray-700 border-gray-600 focus:ring-blue-600 ring-offset-gray-800 focus:ring-offset-gray-800" required />
          </div>
          <label htmlFor="remember" className="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300">Remember me</label>
        </div>

        <button className="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800" type="submit" disabled={loading}>
          {loading ? "loading" : "Sign Up"}
        </button>
      </form>
    </div>
  </>);
}

export default SignupForm;
