import { jwtDecode } from "jwt-decode";
import React, { useEffect, useState } from "react";
import Swal from "sweetalert2";
import { getCustomerDetail } from "../../utils/api";
import EditCustomer from "./EditCustomer";

const CustomerProfile = () => {
  const [customer, setCustomer] = useState(null);
  const [isEditing, setIsEditing] = useState(false);

  const loadCustomer = async () => {
    try {
      const token = localStorage.getItem("token");
      const decoded = jwtDecode(token);
      const customerId = decoded.sub;

      const data = await getCustomerDetail(customerId, token);
      setCustomer({ ...data, id: customerId });
    } catch (err) {
      Swal.fire({
        icon: "error",
        title: "Failed to fetch customer detail",
        text: err.message,
      });
    }
  };

  useEffect(() => {
    loadCustomer();
  }, []);

  if (!customer) return <p>Loading customer...</p>;

  return (
    <div className="customer-profile">
      <h2>Customer Profile</h2>
      {!isEditing ? (
        <div>
          <p>
            <strong>Name:</strong> {customer.name}
          </p>
          <p>
            <strong>Email:</strong> {customer.email}
          </p>
          <p>
            <strong>Phone:</strong> {customer.phone}
          </p>
          <p>
            <strong>Wa Link:</strong> {customer.wa_link}
          </p>
          <p>
            <strong>Intro:</strong> {customer.intro}
          </p>
          <p>
            <strong>About:</strong> {customer.about}
          </p>
          <p>
            <strong>Profile Picture:</strong> {customer.profile_picture}
          </p>
          <p>
            <strong>Created At:</strong>{" "}
            {new Date(customer.createdAt).toLocaleString()}
          </p>
          <button onClick={() => setIsEditing(true)}>Edit Profile</button>
        </div>
      ) : (
        <EditCustomer
          customer={customer}
          setIsEditing={setIsEditing}
          reloadCustomer={loadCustomer}
        />
      )}
    </div>
  );
};

export default CustomerProfile;
