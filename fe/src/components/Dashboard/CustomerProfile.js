import { jwtDecode } from "jwt-decode";
import React, { useEffect, useState } from "react";
import Swal from "sweetalert2";
import { getCustomerDetail, getIdFromToken } from "../../utils/api";
import EditCustomer from "./EditCustomer";
import Logout from "../Logout";

const CustomerProfile = ({ setIsAuthenticated }) => {
  const [customer, setCustomer] = useState(null);
  const [isEditing, setIsEditing] = useState(false);

  const loadCustomer = async () => {
    try {
      const { id, token } = getIdFromToken();

      const data = await getCustomerDetail(token);
      setCustomer({ ...data, id: id });
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
      <h2>Profile</h2>
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
            <strong>API KEY:</strong> {customer.api_key}
          </p>
          <div style={{ marginTop: "30px", marginBottom: "18px" }}>
            <button onClick={() => setIsEditing(true)}>Edit Profile</button>
            <Logout setIsAuthenticated={setIsAuthenticated} />
          </div>
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
