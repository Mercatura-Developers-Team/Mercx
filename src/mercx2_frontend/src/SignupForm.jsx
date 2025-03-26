import React, { useEffect, useState } from "react";
import { useAuth } from "./use-auth-client";
import { useFormik } from "formik";
import * as Yup from "yup";
import { useNavigate } from 'react-router-dom';  // Ensure useNavigate is imported


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

  referredBy: Yup.string().optional(),

});

const SignupForm = () => {
  const { isAuthenticated, kycActor ,principal} = useAuth();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");
  const navigate = useNavigate();  // Use navigate for redirection

  // Check KYC Status
  useEffect(() => {
    const checkUser = async () => {
      if (isAuthenticated && principal) {
        try {
          const exists = await kycActor.has_username_for_principal(principal);
          if (exists) {
            // Redirect if user is already registered
            navigate('/');
          }
        } catch (error) {
          console.error("Error checking user existence:", error);
        }
      }
    };
    checkUser();
  }, [isAuthenticated, principal]);

  const formik = useFormik({
    initialValues: {
      username: "",
      fullname: "",
      phone: "",
      email: "",
      referredBy: "",
    },
    validationSchema: SignupSchema,
    onSubmit: async (values) => {
      setLoading(true);
      setError("");

      // Correctly format the `referredBy` field for Candid (optional text)
      const referral = values.referredBy && values.referredBy.trim() !== ""
        ? [values.referredBy.trim()]  // ✅ Wrap in an array for `Some(value)`
        : [];  // ✅ Use an empty array for `None`

      try {
        const response = await kycActor.signup({
          username: values.username,
          full_name: values.fullname,
          phone_number: values.phone,
          email: values.email,
          refered_by: referral,  // ✅ Properly formatted
        });

        if (response && response.Ok) {
          alert("Signup successful!");
          navigate('/');
        } else {
          throw new Error(response.Err || "Signup failed");
        }
      } catch (err) {
        setError(err.message || "Signup failed. Please try again.");
      } finally {
        setLoading(false);
      }
    },
  });

  if (!isAuthenticated) {
    return (<div className="min-h-screen bg-gray-900 ">
      {/* <p className="  min-h-screen bg-gray-900 text-white ">Please connect your wallet to sign up.</p> */}
      <div class=" flex items-center justify-center p-4  mb-4 text-sm  border rounded-lg bg-gray-800 text-blue-400 border-blue-800" role="alert">
        <svg class="shrink-0 inline w-4 h-4 me-3" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 20 20">
          <path d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5ZM9.5 4a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3ZM12 15H8a1 1 0 0 1 0-2h1v-3H8a1 1 0 0 1 0-2h2a1 1 0 0 1 1 1v4h1a1 1 0 0 1 0 2Z" />
        </svg>
        <span class="sr-only">Info</span>
        <div>
          Please connect to wallet to sign up.
        </div>
      </div>

    </div>)
      ;
  }
  return (<>
    <div className="relative overflow-hidden min-h-screen border-t-[1px] border-slate-800 bg-gray-900">

      <div className="max-w-md mx-auto rounded-3xl bg-slate-800 p-6 shadow-md m-20">
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
              placeholder=" "
            />
            <label htmlFor="username" className="peer-focus:font-medium absolute text-sm  text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 rtl:peer-focus:translate-x-1/4 peer-focus:text-blue-500  peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6">Username</label>
            {formik.touched.username && formik.errors.username && (
              <p className="text-red-400 text-xs">{formik.errors.username}</p>
            )}
          </div>
          <div className="relative z-0 w-full mb-5 group">



            <input
              id="fullname"
              name="fullname"
              type="text"
              onChange={formik.handleChange}
              onBlur={formik.handleBlur}
              value={formik.values.fullname}
              className="block py-2.5 px-0 w-full text-sm bg-transparent border-0 border-b-2  appearance-none text-white border-gray-600 focus:border-blue-500 focus:outline-none focus:ring-0  peer" placeholder=" "
            />
            <label htmlFor="fullname" className="peer-focus:font-medium absolute text-sm  text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto  peer-focus:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6">
              FullName</label>
            {formik.touched.fullname && formik.errors.fullname && (
              <p className="text-red-400 text-xs">{formik.errors.fullname}</p>
            )}

          </div>
          <div className="relative z-0 w-full mb-5 group">
            <input type="tel" name="phone" id="phone" className="block py-2.5 px-0 w-full text-sm  bg-transparent border-0 border-b-2 appearance-none text-white border-gray-600 focus:border-blue-500 focus:outline-none focus:ring-0  peer" placeholder=" " onChange={formik.handleChange}
              onBlur={formik.handleBlur}
              value={formik.values.phone} />
            <label htmlFor="phone" className="peer-focus:font-medium absolute text-sm  text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 rtl:peer-focus:translate-x-1/4  peer-focus:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6">Phone number </label>
            {formik.touched.phone && formik.errors.phone && (
              <p className="text-red-400 text-xs">{formik.errors.phone}</p>
            )}
          </div>
          <div className="relative z-0 w-full mb-5 group">
            <input type="email" name="email" id="email"
              onChange={formik.handleChange}
              onBlur={formik.handleBlur}
              value={formik.values.email}
              className="block py-2.5 px-0 w-full text-sm  bg-transparent border-0 border-b-2  appearance-none text-white border-gray-600  focus:outline-none focus:ring-0 focus:border-blue-600 peer" placeholder=" " required />
            <label htmlFor="email" className="peer-focus:font-medium absolute text-sm  text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto  peer-focus:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6">Email address</label>
            {formik.touched.email && formik.errors.email && (
              <p className="text-red-400 text-xs">{formik.errors.email}</p>
            )}
          </div>

          <div className="relative z-0 w-full mb-5 group">
            <input
              id="referredBy"
              name="referredBy"
              type="text"
              onChange={formik.handleChange}
              onBlur={formik.handleBlur}
              value={formik.values.referredBy}
              className="block py-2.5 px-0 w-full text-sm bg-transparent border-0 border-b-2 text-white border-gray-600 focus:border-blue-500 focus:outline-none focus:ring-0 peer"
              placeholder=" "
            />
            <label
              htmlFor="referredBy"
              className="peer-focus:font-medium absolute text-sm text-gray-400 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6"
            >
              Referral Username (optional)
            </label>
            {formik.touched.referredBy && formik.errors.referredBy && (
              <p className="text-red-400 text-xs">{formik.errors.referredBy}</p>
            )}
          </div>
          <button className="bg-gradient-to-r-indigo-500-700 hover:bg-gradient-to-r-indigo-700-darker text-white py-2 px-4 font-bold rounded-lg text-sm flex items-center justify-center" type="submit" disabled={loading}>
            {loading ? (
              <>
                <span>Loading</span>
                <svg className="animate-spin h-5 w-5 ml-2" viewBox="0 0 24 24">
                  <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                  <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.86 1.861 7.298 4.708 9.291l1.292-1.292z"></path>
                </svg>
              </>
            ) : "Sign Up"}
          </button>

        </form>
      </div>
    </div>
  </>);
}

export default SignupForm;
