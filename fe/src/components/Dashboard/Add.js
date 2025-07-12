import React, { useState } from "react";
import Swal from "sweetalert2";
import { addExperience } from "../../utils/api";

const Add = ({ setIsAdding, reloadExperiences }) => {
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
      !endDate ||
      !position ||
      !description ||
      !order
    ) {
      return Swal.fire({
        icon: "error",
        title: "Error!",
        text: "All fields are required.",
        showConfirmButton: true,
      });
    }

    const token = localStorage.getItem("token");

    const payload = {
      company,
      work_type: workType,
      location,
      start_date: new Date(startDate).toISOString(),
      end_date: new Date(endDate).toISOString(),
      is_present: isPresent,
      position,
      description,
      order: Number(order),
    };

    try {
      const newExperience = await addExperience(payload, token);

      Swal.fire({
        icon: "success",
        title: "Added!",
        text: `${company}'s experience has been added.`,
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

export default Add;
