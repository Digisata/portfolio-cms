import React, { useState } from "react";
import Swal from "sweetalert2";
import { updateCustomerById } from "../../utils/api";

const EditCustomer = ({ customer, setIsEditing, reloadCustomer }) => {
  const [name, setName] = useState(customer.name);
  const [email, setEmail] = useState(customer.email);
  const [phone, setPhone] = useState(customer.phone);
  const [waLink, setWaLink] = useState(customer.wa_link);
  const [intro, setIntro] = useState(customer.intro);
  const [about, setAbout] = useState(customer.about);
  const [profilePicture, setProfilePicture] = useState(
    customer.profile_picture,
  );

  const handleUpdate = async (e) => {
    e.preventDefault();
    const token = localStorage.getItem("token");

    const payload = {
      name,
      email,
      phone,
      wa_link: waLink,
      intro,
      about,
      profile_picture: profilePicture,
      password: customer.password,
    };

    try {
      await updateCustomerById(customer.id, payload, token);
      await reloadCustomer();
      Swal.fire({
        icon: "success",
        title: "Updated!",
        text: "Customer profile has been updated.",
        timer: 1500,
        showConfirmButton: false,
      });
      setIsEditing(false);
    } catch (err) {
      Swal.fire({
        icon: "error",
        title: "Failed to update",
        text: err.message,
      });
    }
  };

  return (
    <form onSubmit={handleUpdate} className="edit-form">
      <h2>Edit Customer</h2>
      <label>Name</label>
      <input
        type="text"
        value={name}
        onChange={(e) => setName(e.target.value)}
      />

      <label>Email</label>
      <input
        type="email"
        value={email}
        onChange={(e) => setEmail(e.target.value)}
      />

      <label>Phone</label>
      <input
        type="phone"
        value={phone}
        onChange={(e) => setPhone(e.target.value)}
      />

      <label>Wa Link</label>
      <input
        type="text"
        value={waLink}
        onChange={(e) => setWaLink(e.target.value)}
      />

      <label>Intro</label>
      <textarea
        name="description"
        value={intro}
        onChange={(e) => setIntro(e.target.value)}
      />

      <label>About</label>
      <textarea
        name="description"
        value={about}
        onChange={(e) => setAbout(e.target.value)}
      />

      <label>Profile Picture</label>
      <input
        type="text"
        value={profilePicture}
        onChange={(e) => setProfilePicture(e.target.value)}
      />

      <div style={{ marginTop: "20px" }}>
        <button type="submit">Save</button>
        <button
          type="button"
          onClick={() => setIsEditing(false)}
          style={{ marginLeft: "12px" }}
        >
          Cancel
        </button>
      </div>
    </form>
  );
};

export default EditCustomer;
