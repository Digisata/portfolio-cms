import React, { useState } from "react";
import Swal from "sweetalert2";
import { updateExperienceById } from "../../utils/api";

const Edit = ({ selectedExperience, reloadExperiences, setIsEditing }) => {
  const [formData, setFormData] = useState({ ...selectedExperience });

  const handleInputChange = (e) => {
    const { name, value, type, checked } = e.target;
    setFormData((prev) => ({
      ...prev,
      [name]: type === "checkbox" ? checked : value,
    }));
  };

  const handleSubmit = async (e) => {
    e.preventDefault();

    const token = localStorage.getItem("token");

    const payload = {
      company: formData.company,
      work_type: formData.work_type,
      location: formData.location,
      start_date: new Date(formData.start_date).toISOString(),
      end_date: new Date(formData.end_date).toISOString(),
      is_present: formData.is_present,
      position: formData.position,
      description: formData.description,
      order: Number(formData.order),
    };

    try {
      await updateExperienceById(formData.id, payload, token);
      await reloadExperiences();
      Swal.fire({
        icon: "success",
        title: "Updated!",
        text: "Experience has been updated.",
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
    <form onSubmit={handleSubmit} className="edit-form">
      <h2>Edit Experience</h2>

      <label>Company</label>
      <input
        name="company"
        value={formData.company}
        onChange={handleInputChange}
      />

      <label>Work Type</label>
      <input
        name="work_type"
        value={formData.work_type}
        onChange={handleInputChange}
      />

      <label>Location</label>
      <input
        name="location"
        value={formData.location}
        onChange={handleInputChange}
      />

      <label>Position</label>
      <input
        name="position"
        value={formData.position}
        onChange={handleInputChange}
      />

      <label>Start Date</label>
      <input
        type="datetime-local"
        name="start_date"
        value={formData.start_date}
        onChange={handleInputChange}
      />

      <label>End Date</label>
      <input
        type="datetime-local"
        name="end_date"
        value={formData.end_date}
        onChange={handleInputChange}
      />

      <label>Is Present</label>
      <input
        type="checkbox"
        name="is_present"
        checked={formData.is_present}
        onChange={handleInputChange}
      />

      <label>Description</label>
      <textarea
        name="description"
        value={formData.description}
        onChange={handleInputChange}
      />

      <label>Order</label>
      <input
        name="order"
        type="number"
        value={formData.order}
        onChange={handleInputChange}
      />

      <div>
        <button type="submit">Update</button>
        <button onClick={() => setIsEditing(false)}>Cancel</button>
      </div>
    </form>
  );
};

export default Edit;
