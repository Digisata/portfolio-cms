import React, { useState } from "react";
import Swal from "sweetalert2";
import {
  updateExperienceById,
  updateProjectById,
  updateSkillById,
  updateSocialById,
} from "../../utils/api";

const EditExperience = ({
  selectedExperience,
  reloadExperiences,
  setIsEditing,
}) => {
  const formatToLocalInput = (utcString) => {
    const date = new Date(utcString);
    return date.toISOString().slice(0, 16); // "YYYY-MM-DDTHH:MM"
  };

  const [formData, setFormData] = useState({
    ...selectedExperience,
    is_present: true,
    start_date: formatToLocalInput(selectedExperience.start_date),
    end_date:
      selectedExperience.end_date &&
      formatToLocalInput(selectedExperience.end_date),
    description: Array.isArray(selectedExperience.description)
      ? selectedExperience.description
      : null,
  });

  const handleDescriptionChange = (index, value) => {
    setFormData((prev) => {
      const updatedDescriptions = [...prev.description];
      updatedDescriptions[index] = value;
      return { ...prev, description: updatedDescriptions };
    });
  };

  const addDescriptionField = () => {
    setFormData((prev) => ({
      ...prev,
      description: prev.description ? [...prev.description, ""] : [""],
    }));
  };

  const removeDescriptionField = (index) => {
    setFormData((prev) => {
      const updatedDescriptions = prev.description.filter(
        (_, i) => i !== index
      );
      return { ...prev, description: updatedDescriptions };
    });
  };

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
      end_date: formData.end_date
        ? new Date(formData.end_date).toISOString()
        : null,
      position: formData.position,
      description: formData.description,
      order: Number(formData.order),
    };

    try {
      await updateExperienceById(formData._id || formData.id, payload, token);
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

      {formData.end_date && (
        <>
          <label>End Date</label>
          <input
            type="datetime-local"
            name="end_date"
            value={formData.end_date}
            onChange={handleInputChange}
            disabled
          />
        </>
      )}

      {!formData.end_date && (
        <>
          <label>Is Present</label>
          <input
            type="checkbox"
            name="is_present"
            checked={!formData.end_date}
            onChange={handleInputChange}
            disabled
          />
        </>
      )}

      <label>Description</label>
      {formData.description &&
        formData.description.map((desc, index) => (
          <div
            key={index}
            style={{
              display: "flex",
              alignItems: "center",
              marginBottom: "8px",
            }}
          >
            <input
              type="text"
              value={desc}
              onChange={(e) => handleDescriptionChange(index, e.target.value)}
              style={{ flex: 1 }}
            />
            <button
              type="button"
              onClick={() => removeDescriptionField(index)}
              style={{
                marginLeft: "8px",
                padding: "4px 8px",
                background: "#ff4d4f",
                color: "#fff",
                border: "none",
                borderRadius: "4px",
                cursor: "pointer",
              }}
            >
              ❌
            </button>
          </div>
        ))}
      <button
        type="button"
        onClick={addDescriptionField}
        style={{
          marginTop: "8px",
          padding: "6px 12px",
          background: "#1890ff",
          color: "#fff",
          border: "none",
          borderRadius: "4px",
          cursor: "pointer",
        }}
      >
        ➕ Add Description
      </button>

      <label>Order</label>
      <input
        name="order"
        type="number"
        value={formData.order}
        onChange={handleInputChange}
      />

      <div>
        <div style={{ marginTop: "30px" }}>
          <input type="submit" value="Update" />
          <input
            type="button"
            className="muted-button"
            value="Cancel"
            style={{ marginLeft: "12px" }}
            onClick={() => setIsEditing(false)}
          />
        </div>
      </div>
    </form>
  );
};

const EditProject = ({
  selectedProject,
  reloadProjects,
  setIsEditingProject,
}) => {
  const [formData, setFormData] = useState({
    ...selectedProject,
    stack: Array.isArray(selectedProject.stack) ? selectedProject.stack : null,
  });

  const handleStackChange = (index, value) => {
    setFormData((prev) => {
      const updatedStacks = [...prev.stack];
      updatedStacks[index] = value;
      return { ...prev, stack: updatedStacks };
    });
  };

  const addStackField = () => {
    setFormData((prev) => ({
      ...prev,
      stack: [...prev.stack, ""],
    }));
  };

  const removeStackField = (index) => {
    setFormData((prev) => {
      const updatedStacks = prev.stack.filter((_, i) => i !== index);
      return { ...prev, stack: updatedStacks };
    });
  };

  const handleInputChange = (e) => {
    const { name, value } = e.target;
    setFormData((prev) => ({
      ...prev,
      [name]: value,
    }));
  };

  const handleSubmit = async (e) => {
    e.preventDefault();

    const token = localStorage.getItem("token");

    const payload = {
      name: formData.name,
      description: formData.description,
      link: formData.link,
      photo_link: formData.photo_link,
      order: Number(formData.order),
      stack: formData.stack,
    };

    try {
      await updateProjectById(formData._id, payload, token);
      await reloadProjects();
      Swal.fire({
        icon: "success",
        title: "Updated!",
        text: "Project has been updated.",
        timer: 1500,
        showConfirmButton: false,
      });
      setIsEditingProject(false);
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
      <h2>Edit Project</h2>

      <label>Name</label>
      <input name="name" value={formData.name} onChange={handleInputChange} />

      <label>Description</label>
      <textarea
        name="description"
        value={formData.description}
        onChange={handleInputChange}
      />

      <label>Link</label>
      <input name="link" value={formData.link} onChange={handleInputChange} />

      <label>Photo Link</label>
      <input
        name="photo_link"
        value={formData.photo_link}
        onChange={handleInputChange}
      />

      <label>Order</label>
      <input
        type="number"
        name="order"
        value={formData.order}
        onChange={handleInputChange}
      />

      <label>Stack</label>
      {formData.stack &&
        formData.stack.map((stack, index) => (
          <div
            key={index}
            style={{
              display: "flex",
              alignItems: "center",
              marginBottom: "8px",
            }}
          >
            <input
              type="text"
              value={stack}
              onChange={(e) => handleStackChange(index, e.target.value)}
              style={{ flex: 1 }}
            />
            <button
              type="button"
              onClick={() => removeStackField(index)}
              style={{
                marginLeft: "8px",
                padding: "4px 8px",
                background: "#ff4d4f",
                color: "#fff",
                border: "none",
                borderRadius: "4px",
                cursor: "pointer",
              }}
            >
              ❌
            </button>
          </div>
        ))}
      <button
        type="button"
        onClick={addStackField}
        style={{
          marginTop: "8px",
          padding: "6px 12px",
          background: "#1890ff",
          color: "#fff",
          border: "none",
          borderRadius: "4px",
          cursor: "pointer",
        }}
      >
        ➕ Add Stack
      </button>

      <div>
        <div style={{ marginTop: "30px" }}>
          <input type="submit" value="Update" />
          <input
            type="button"
            className="muted-button"
            value="Cancel"
            style={{ marginLeft: "12px" }}
            onClick={() => setIsEditingProject(false)}
          />
        </div>
      </div>
    </form>
  );
};

const EditSkill = ({ selectedSkill, reloadSkills, setIsEditingSkill }) => {
  const [formData, setFormData] = useState({ ...selectedSkill });

  const handleInputChange = (e) => {
    const { name, value } = e.target;
    setFormData((prev) => ({
      ...prev,
      [name]: value,
    }));
  };

  const handleStackChange = (e) => {
    const value = e.target.value;
    setFormData((prev) => ({
      ...prev,
      stack: value.split(",").map((s) => s.trim()),
    }));
  };

  const handleSubmit = async (e) => {
    e.preventDefault();

    const token = localStorage.getItem("token");

    const payload = {
      name: formData.name,
      order: Number(formData.order),
    };

    try {
      await updateSkillById(formData._id, payload, token);
      await reloadSkills();
      Swal.fire({
        icon: "success",
        title: "Updated!",
        text: "Skill has been updated.",
        timer: 1500,
        showConfirmButton: false,
      });
      setIsEditingSkill(false);
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
      <h2>Edit Skill</h2>

      <label>Name</label>
      <input name="name" value={formData.name} onChange={handleInputChange} />

      <label>Order</label>
      <input
        type="number"
        name="order"
        value={formData.order}
        onChange={handleInputChange}
      />

      <div>
        <div style={{ marginTop: "30px" }}>
          <input type="submit" value="Update" />
          <input
            type="button"
            className="muted-button"
            value="Cancel"
            style={{ marginLeft: "12px" }}
            onClick={() => setIsEditingSkill(false)}
          />
        </div>
      </div>
    </form>
  );
};

const EditSocial = ({ selectedSocial, reloadSocials, setIsEditingSocial }) => {
  const [formData, setFormData] = useState({ ...selectedSocial });

  const handleInputChange = (e) => {
    const { name, value } = e.target;
    setFormData((prev) => ({
      ...prev,
      [name]: value,
    }));
  };

  const handleStackChange = (e) => {
    const value = e.target.value;
    setFormData((prev) => ({
      ...prev,
      stack: value.split(",").map((s) => s.trim()),
    }));
  };

  const handleSubmit = async (e) => {
    e.preventDefault();

    const token = localStorage.getItem("token");

    const payload = {
      name: formData.name,
      link: formData.link,
      order: Number(formData.order),
    };

    try {
      await updateSocialById(formData._id, payload, token);
      await reloadSocials();
      Swal.fire({
        icon: "success",
        title: "Updated!",
        text: "Social has been updated.",
        timer: 1500,
        showConfirmButton: false,
      });
      setIsEditingSocial(false);
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
      <h2>Edit Social</h2>

      <label>Name</label>
      <input name="name" value={formData.name} onChange={handleInputChange} />

      <label>Link</label>
      <input name="link" value={formData.link} onChange={handleInputChange} />

      <label>Order</label>
      <input
        type="number"
        name="order"
        value={formData.order}
        onChange={handleInputChange}
      />

      <div>
        <div style={{ marginTop: "30px" }}>
          <input type="submit" value="Update" />
          <input
            type="button"
            className="muted-button"
            value="Cancel"
            style={{ marginLeft: "12px" }}
            onClick={() => setIsEditingSocial(false)}
          />
        </div>
      </div>
    </form>
  );
};

export { EditExperience, EditProject, EditSkill, EditSocial };
