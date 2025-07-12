import React, { useState, useEffect } from "react";
import Swal from "sweetalert2";

import Header from "./Header";
import ExperienceTable from "./Table";
import Add from "./Add";
import Edit from "./Edit";

import {
  getExperienceDetail,
  deleteExperienceById,
  apiRequest,
} from "../../utils/api";

const Dashboard = ({ setIsAuthenticated }) => {
  const [experiences, setExperiences] = useState([]);
  const [selectedExperience, setSelectedExperience] = useState(null);
  const [isAdding, setIsAdding] = useState(false);
  const [isEditing, setIsEditing] = useState(false);
  const [error, setError] = useState("");

  const loadExperiences = async () => {
    try {
      const data = await apiRequest("/experience");
      setExperiences(data);
    } catch (err) {
      setError(err.message || "Failed to load experiences");
    }
  };

  useEffect(() => {
    loadExperiences();
  }, []);

  const handleEdit = async (id) => {
    try {
      const token = localStorage.getItem("token");
      const data = await getExperienceDetail(id, token);
      setSelectedExperience({ ...data, id });
      setIsEditing(true);
    } catch (err) {
      Swal.fire({
        icon: "error",
        title: "Failed to fetch detail",
        text: err.message,
      });
    }
  };

  const handleDelete = (id) => {
    Swal.fire({
      icon: "warning",
      title: "Are you sure?",
      text: "You won't be able to revert this!",
      showCancelButton: true,
      confirmButtonText: "Yes, delete it!",
      cancelButtonText: "No, cancel!",
    }).then(async (result) => {
      if (result.isConfirmed) {
        const token = localStorage.getItem("token");

        try {
          const deleted = await deleteExperienceById(id, token);

          Swal.fire({
            icon: "success",
            title: "Deleted!",
            text: `${deleted.company}'s experience has been deleted.`,
            showConfirmButton: false,
            timer: 1500,
          });

          // 🔁 Refresh the list from backend
          await loadExperiences();
        } catch (err) {
          Swal.fire({
            icon: "error",
            title: "Delete Failed",
            text: err.message,
          });
        }
      }
    });
  };

  return (
    <div className="container">
      {!isAdding && !isEditing && (
        <>
          <Header
            setIsAdding={setIsAdding}
            setIsAuthenticated={setIsAuthenticated}
          />
          <ExperienceTable
            experiences={experiences}
            handleEdit={handleEdit}
            handleDelete={handleDelete}
            error={error}
          />
        </>
      )}
      {isAdding && (
        <Add
          experiences={experiences}
          setExperiences={setExperiences}
          setIsAdding={setIsAdding}
          reloadExperiences={loadExperiences}
        />
      )}
      {isEditing && (
        <Edit
          experiences={experiences}
          selectedExperience={selectedExperience}
          setExperiences={setExperiences}
          setIsEditing={setIsEditing}
          reloadExperiences={loadExperiences}
        />
      )}
    </div>
  );
};

export default Dashboard;
