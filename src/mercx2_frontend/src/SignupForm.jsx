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
  });

const SignupForm = () => {
    const { isAuthenticated , kycActor} = useAuth();
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
    },
    validationSchema: SignupSchema,
    onSubmit: async (values) => {
      setLoading(true);
      setError("");

      try {
        const response = await kycActor.signup({
          username: values.username,
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
    return ( <> 
     <div className="max-w-md mx-auto bg-gray-900 p-6 rounded shadow-md m-20">
      <h2 className="text-2xl font-bold text-white mb-4">Signup</h2>

      {error && <p className="text-red-500">{error}</p>}

      <form onSubmit={formik.handleSubmit} className="max-w-sm mx-auto">
        <div >
          <label htmlFor="username" className="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Username</label>
          <input
            id="username"
            name="username"
            type="text"
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
            value={formik.values.username}
            className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
          />
          {formik.touched.username && formik.errors.username && (
            <p className="text-red-400 text-sm">{formik.errors.username}</p>
          )}
        </div>

        <div className="flex items-start mb-5">
    <div className="flex items-center h-5">
      <input id="remember" type="checkbox" value="" className="w-4 h-4 border border-gray-300 rounded-sm bg-gray-50 focus:ring-3 focus:ring-blue-300 dark:bg-gray-700 dark:border-gray-600 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800" required />
    </div>
    <label htmlFor="remember" className="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300">Remember me</label>
  </div>

        <button className="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800" type="submit" disabled={loading}>
          {loading ? "loading": "Sign Up"}
        </button>
      </form>
    </div>
    </>);
}
 
export default SignupForm;
