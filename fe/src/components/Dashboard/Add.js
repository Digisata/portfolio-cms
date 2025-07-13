import React, { useState } from "react";
import Swal from "sweetalert2";
import {
  addExperience,
  addProject,
  addSkill,
  addSocial,
} from "../../utils/api";

const AddExperience = ({ setIsAdding, reloadExperiences }) => {
  const [company, setCompany] = useState("");
  const [workType, setWorkType] = useState("");
  const [location, setLocation] = useState("");
  const [startDate, setStartDate] = useState("");
  const [endDate, setEndDate] = useState("");
  const [isPresent, setIsPresent] = useState(false);
  const [position, setPosition] = useState("");
  const [description, setDescription] = useState("");
  const [order, setOrder] = useState(1);

  const handleAdd = async (e) => {
    e.preventDefault();

    if (
      !company ||
      !workType ||
      !location ||
      !startDate ||
      !position ||
      !description ||
      !order ||
      (!isPresent && !endDate)
    ) {
      return Swal.fire({
        icon: "error",
        title: "Error!",
        text: "All fields are required.",
        showConfirmButton: true,
      });
    }

    const token = localStorage.getItem("token");

    let payload = {
      company,
      work_type: workType,
      location,
      start_date: new Date(startDate).toISOString(),
      is_present: isPresent,
      position,
      description,
      order: Number(order),
    };

    if (endDate) {
      payload.end_date = new Date(endDate).toISOString();
    }

    if (isPresent) {
      payload.end_date = new Date().toISOString();
    }

    try {
      const newExperience = await addExperience(payload, token);

      Swal.fire({
        icon: "success",
        title: "Added!",
        text: `experience has been added.`,
        showConfirmButton: false,
        timer: 1500,
      });

      // Reload experience list
      await reloadExperiences();
      setIsAdding(false);
    } catch (err) {
      Swal.fire({
        icon: "error",
        title: "Failed to add experience",
        text: err.message,
      });
    }
  };

  return (
    <div className="small-container">
      <form onSubmit={handleAdd}>
        <h1>Add Experience</h1>

        <label>Company</label>
        <input
          type="text"
          value={company}
          onChange={(e) => setCompany(e.target.value)}
        />

        <label>Work Type</label>
        <input
          type="text"
          value={workType}
          onChange={(e) => setWorkType(e.target.value)}
        />

        <label>Location</label>
        <input
          type="text"
          value={location}
          onChange={(e) => setLocation(e.target.value)}
        />

        <label>Position</label>
        <input
          type="text"
          value={position}
          onChange={(e) => setPosition(e.target.value)}
        />

        <label>Start Date</label>
        <input
          type="datetime-local"
          value={startDate}
          onChange={(e) => setStartDate(e.target.value)}
        />

        <label>End Date</label>
        <input
          type="datetime-local"
          value={endDate}
          onChange={(e) => setEndDate(e.target.value)}
        />

        <label>Currently Working?</label>
        <input
          type="checkbox"
          checked={isPresent}
          onChange={(e) => setIsPresent(e.target.checked)}
        />

        <label>Description</label>
        <textarea
          value={description}
          onChange={(e) => setDescription(e.target.value)}
        />

        <label>Order</label>
        <input
          type="number"
          value={order}
          onChange={(e) => setOrder(e.target.value)}
        />

        <div style={{ marginTop: "30px" }}>
          <input type="submit" value="Add" />
          <input
            style={{ marginLeft: "12px" }}
            className="muted-button"
            type="button"
            value="Cancel"
            onClick={() => setIsAdding(false)}
          />
        </div>
      </form>
    </div>
  );
};

const AddProject = ({ setIsAddingProject, reloadProjects }) => {
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [link, setLink] = useState("");
  const [photoLink, setPhotoLink] = useState("");
  const [order, setOrder] = useState(1);
  const [stack, setStack] = useState("");

  const handleAdd = async (e) => {
    e.preventDefault();

    if (!name || !description || !link || !photoLink || !order || !stack) {
      return Swal.fire({
        icon: "error",
        title: "Error!",
        text: "All fields are required.",
        showConfirmButton: true,
      });
    }

    const token = localStorage.getItem("token");

    const payload = {
      name,
      description,
      link,
      photo_link: photoLink,
      order: Number(order),
      stack: stack.split(",").map((s) => s.trim()), // Convert comma-separated string to array
    };

    try {
      await addProject(payload, token);

      Swal.fire({
        icon: "success",
        title: "Added!",
        text: `project has been added.`,
        showConfirmButton: false,
        timer: 1500,
      });

      await reloadProjects();
      setIsAddingProject(false);
    } catch (err) {
      Swal.fire({
        icon: "error",
        title: "Failed to add project",
        text: err.message,
      });
    }
  };

  return (
    <div className="small-container">
      <form onSubmit={handleAdd}>
        <h1>Add Project</h1>

        <label>Name</label>
        <input value={name} onChange={(e) => setName(e.target.value)} />

        <label>Description</label>
        <textarea
          value={description}
          onChange={(e) => setDescription(e.target.value)}
        />

        <label>Link</label>
        <input value={link} onChange={(e) => setLink(e.target.value)} />

        <label>Photo Link</label>
        <input
          value={photoLink}
          onChange={(e) => setPhotoLink(e.target.value)}
        />

        <label>Order</label>
        <input
          type="number"
          value={order}
          onChange={(e) => setOrder(e.target.value)}
        />

        <label>Stack (comma-separated)</label>
        <input value={stack} onChange={(e) => setStack(e.target.value)} />

        <div style={{ marginTop: "30px" }}>
          <input type="submit" value="Add" />
          <input
            type="button"
            className="muted-button"
            value="Cancel"
            style={{ marginLeft: "12px" }}
            onClick={() => setIsAddingProject(false)}
          />
        </div>
      </form>
    </div>
  );
};

const AddSkill = ({ setIsAddingSkill, reloadSkills }) => {
  const [name, setName] = useState("");
  const [order, setOrder] = useState(1);

  const handleAdd = async (e) => {
    e.preventDefault();

    if (!name || !order) {
      return Swal.fire({
        icon: "error",
        title: "Error!",
        text: "All fields are required.",
        showConfirmButton: true,
      });
    }

    const token = localStorage.getItem("token");

    const payload = {
      name,
      order: Number(order),
    };

    try {
      await addSkill(payload, token);

      Swal.fire({
        icon: "success",
        title: "Added!",
        text: `skill has been added.`,
        showConfirmButton: false,
        timer: 1500,
      });

      await reloadSkills();
      setIsAddingSkill(false);
    } catch (err) {
      Swal.fire({
        icon: "error",
        title: "Failed to add skill",
        text: err.message,
      });
    }
  };

  return (
    <div className="small-container">
      <form onSubmit={handleAdd}>
        <h1>Add Skill</h1>

        <label>Name</label>
        <input value={name} onChange={(e) => setName(e.target.value)} />

        <label>Order</label>
        <input
          type="number"
          value={order}
          onChange={(e) => setOrder(e.target.value)}
        />

        <div style={{ marginTop: "30px" }}>
          <input type="submit" value="Add" />
          <input
            type="button"
            className="muted-button"
            value="Cancel"
            style={{ marginLeft: "12px" }}
            onClick={() => setIsAddingSkill(false)}
          />
        </div>
      </form>
    </div>
  );
};

const AddSocial = ({ setIsAddingSocial, reloadSocials }) => {
  const [name, setName] = useState("");
  const [link, setLink] = useState("");
  const [order, setOrder] = useState(1);

  const handleAdd = async (e) => {
    e.preventDefault();

    if (!name || !link || !order) {
      return Swal.fire({
        icon: "error",
        title: "Error!",
        text: "All fields are required.",
        showConfirmButton: true,
      });
    }

    const token = localStorage.getItem("token");

    const payload = {
      name,
      link,
      order: Number(order),
    };

    try {
      await addSocial(payload, token);

      Swal.fire({
        icon: "success",
        title: "Added!",
        text: `social has been added.`,
        showConfirmButton: false,
        timer: 1500,
      });

      await reloadSocials();
      setIsAddingSocial(false);
    } catch (err) {
      Swal.fire({
        icon: "error",
        title: "Failed to add social",
        text: err.message,
      });
    }
  };

  return (
    <div className="small-container">
      <form onSubmit={handleAdd}>
        <h1>Add Social</h1>

        <label>Name</label>
        <input value={name} onChange={(e) => setName(e.target.value)} />

        <label>Link</label>
        <input value={link} onChange={(e) => setLink(e.target.value)} />

        <label>Order</label>
        <input
          type="number"
          value={order}
          onChange={(e) => setOrder(e.target.value)}
        />

        <div style={{ marginTop: "30px" }}>
          <input type="submit" value="Add" />
          <input
            type="button"
            className="muted-button"
            value="Cancel"
            style={{ marginLeft: "12px" }}
            onClick={() => setIsAddingSocial(false)}
          />
        </div>
      </form>
    </div>
  );
};

export { AddExperience, AddProject, AddSkill, AddSocial };
