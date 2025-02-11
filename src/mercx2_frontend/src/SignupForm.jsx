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
     <div className="max-w-md mx-auto bg-gray-900 p-6 rounded shadow-md">
      <h2 className="text-2xl font-bold text-white mb-4">Signup</h2>

      {error && <p className="text-red-500">{error}</p>}

      <form onSubmit={formik.handleSubmit} className="flex flex-col gap-4">
        <div>
          <label htmlFor="username" className="text-white">Username</label>
          <input
            id="username"
            name="username"
            type="text"
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
            value={formik.values.username}
            className="bg-gray-800 text-white border-gray-600"
          />
          {formik.touched.username && formik.errors.username && (
            <p className="text-red-400 text-sm">{formik.errors.username}</p>
          )}
        </div>

        <button type="submit" disabled={loading}>
          {loading ? "loading": "Sign Up"}
        </button>
      </form>
    </div>
    </>);
}
 
export default SignupForm;
