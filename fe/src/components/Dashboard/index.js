import React, { useState, useEffect } from "react";
import Swal from "sweetalert2";

import {
  HeaderExperience,
  HeaderProject,
  HeaderSkill,
  HeaderSocial,
} from "./Header";

import {
  ExperienceTable,
  ProjectTable,
  SkillTable,
  SocialTable,
} from "./Table";

import { AddExperience, AddProject, AddSkill, AddSocial } from "./Add";

import { EditExperience, EditProject, EditSkill, EditSocial } from "./Edit";

import {
  getExperienceDetail,
  deleteExperienceById,
  getProjectDetail,
  deleteProjectById,
  getSkillDetail,
  deleteSkillById,
  getSocialDetail,
  deleteSocialById,
  apiRequest,
  getExperience,
  getProject,
  getSkill,
  getSocial,
} from "../../utils/api";

import CustomerProfile from "./CustomerProfile"; // ✅ new import
import Logout from "../Logout";

const Dashboard = ({ setIsAuthenticated }) => {
  const [experiences, setExperiences] = useState([]);
  const [selectedExperience, setSelectedExperience] = useState(null);
  const [isAddingExperience, setIsAddingExperience] = useState(false);
  const [isEditingExperience, setIsEditingExperience] = useState(false);

  const [projects, setProjects] = useState([]);
  const [selectedProject, setSelectedProject] = useState(null);
  const [isAddingProject, setIsAddingProject] = useState(false);
  const [isEditingProject, setIsEditingProject] = useState(false);

  const [skills, setSkills] = useState([]);
  const [selectedSkill, setSelectedSkill] = useState(null);
  const [isAddingSkill, setIsAddingSkill] = useState(false);
  const [isEditingSkill, setIsEditingSkill] = useState(false);

  const [socials, setSocials] = useState([]);
  const [selectedSocial, setSelectedSocial] = useState(null);
  const [isAddingSocial, setIsAddingSocial] = useState(false);
  const [isEditingSocial, setIsEditingSocial] = useState(false);

  const [error, setError] = useState("");

  const loadExperiences = async () => {
    try {
      const token = localStorage.getItem("token");
      const data = await getExperience(token);
      setExperiences(data);
    } catch (err) {
      setError(err.message || "Failed to load experiences");
    }
  };

  const loadProjects = async () => {
    try {
      const token = localStorage.getItem("token");
      const data = await getProject(token);
      setProjects(data);
    } catch (err) {
      setError(err.message || "Failed to load projects");
    }
  };

  const loadSkills = async () => {
    try {
      const token = localStorage.getItem("token");
      const data = await getSkill(token);
      setSkills(data);
    } catch (err) {
      setError(err.message || "Failed to load skills");
    }
  };

  const loadSocials = async () => {
    try {
      const token = localStorage.getItem("token");
      const data = await getSocial(token);
      setSocials(data);
    } catch (err) {
      setError(err.message || "Failed to load socials");
    }
  };

  useEffect(() => {
    loadExperiences();
    loadProjects();
    loadSkills();
    loadSocials();
  }, []);

  const handleEditExperience = async (id) => {
    try {
      const token = localStorage.getItem("token");
      const data = await getExperienceDetail(id, token);
      setSelectedExperience({ ...data, id });
      setIsEditingExperience(true);
    } catch (err) {
      Swal.fire({
        icon: "error",
        title: "Failed to fetch detail",
        text: err.message,
      });
    }
  };

  const handleDeleteExperience = (id) => {
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

  const handleEditProject = async (id) => {
    try {
      const token = localStorage.getItem("token");
      const data = await getProjectDetail(id, token);
      setSelectedProject({ ...data, id });
      setIsEditingProject(true);
    } catch (err) {
      Swal.fire({
        icon: "error",
        title: "Failed to fetch detail",
        text: err.message,
      });
    }
  };

  const handleDeleteProject = (id) => {
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
          await deleteProjectById(id, token);

          Swal.fire({
            icon: "success",
            title: "Deleted!",
            text: `Project has been deleted.`,
            showConfirmButton: false,
            timer: 1500,
          });

          await loadProjects();
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

  const handleEditSkill = async (id) => {
    try {
      const token = localStorage.getItem("token");
      const data = await getSkillDetail(id, token);
      setSelectedSkill({ ...data, id });
      setIsEditingSkill(true);
    } catch (err) {
      Swal.fire({
        icon: "error",
        title: "Failed to fetch detail",
        text: err.message,
      });
    }
  };

  const handleDeleteSkill = (id) => {
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
          await deleteSkillById(id, token);

          Swal.fire({
            icon: "success",
            title: "Deleted!",
            text: `Skill has been deleted.`,
            showConfirmButton: false,
            timer: 1500,
          });

          await loadSkills();
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

  const handleEditSocial = async (id) => {
    try {
      const token = localStorage.getItem("token");
      const data = await getSocialDetail(id, token);
      setSelectedSocial({ ...data, id });
      setIsEditingSocial(true);
    } catch (err) {
      Swal.fire({
        icon: "error",
        title: "Failed to fetch detail",
        text: err.message,
      });
    }
  };

  const handleDeleteSocial = (id) => {
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
          await deleteSocialById(id, token);

          Swal.fire({
            icon: "success",
            title: "Deleted!",
            text: `Social link has been deleted.`,
            showConfirmButton: false,
            timer: 1500,
          });

          await loadSocials();
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
      <div
        style={{
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
        }}
      >
        <h1>Portfolio CMS</h1>
        <Logout setIsAuthenticated={setIsAuthenticated} />
      </div>

      <hr style={{ margin: "40px 0" }} />
      <CustomerProfile setIsAuthenticated={setIsAuthenticated} />

      {/* EXPERIENCE */}
      {!isAddingExperience && !isEditingExperience && (
        <>
          <HeaderExperience
            setIsAdding={setIsAddingExperience}
            setIsAuthenticated={setIsAuthenticated}
          />
          <ExperienceTable
            experiences={experiences}
            handleEdit={handleEditExperience}
            handleDelete={handleDeleteExperience}
            error={error}
          />
        </>
      )}
      {isAddingExperience && (
        <AddExperience
          setIsAdding={setIsAddingExperience}
          reloadExperiences={loadExperiences}
        />
      )}
      {isEditingExperience && (
        <EditExperience
          experiences={experiences}
          selectedExperience={selectedExperience}
          setExperiences={setExperiences}
          setIsEditing={setIsEditingExperience}
          reloadExperiences={loadExperiences}
        />
      )}

      {/* PROJECT */}
      {!isAddingProject && !isEditingProject && (
        <>
          <HeaderProject setIsAddingProject={setIsAddingProject} />
          <ProjectTable
            projects={projects}
            handleEditProject={handleEditProject}
            handleDeleteProject={handleDeleteProject}
            error={error}
          />
        </>
      )}
      {isAddingProject && (
        <AddProject
          setIsAddingProject={setIsAddingProject}
          reloadProjects={loadProjects}
        />
      )}
      {isEditingProject && (
        <EditProject
          selectedProject={selectedProject}
          setIsEditingProject={setIsEditingProject}
          reloadProjects={loadProjects}
        />
      )}

      {/* SKILL */}
      {!isAddingSkill && !isEditingSkill && (
        <>
          <HeaderSkill setIsAddingSkill={setIsAddingSkill} />
          <SkillTable
            skills={skills}
            handleEditSkill={handleEditSkill}
            handleDeleteSkill={handleDeleteSkill}
            error={error}
          />
        </>
      )}
      {isAddingSkill && (
        <AddSkill
          setIsAddingSkill={setIsAddingSkill}
          reloadSkills={loadSkills}
        />
      )}
      {isEditingSkill && (
        <EditSkill
          selectedSkill={selectedSkill}
          setIsEditingSkill={setIsEditingSkill}
          reloadSkills={loadSkills}
        />
      )}

      {/* SOCIAL */}
      {!isAddingSocial && !isEditingSocial && (
        <>
          <HeaderSocial setIsAddingSocial={setIsAddingSocial} />
          <SocialTable
            socials={socials}
            handleEditSocial={handleEditSocial}
            handleDeleteSocial={handleDeleteSocial}
            error={error}
          />
        </>
      )}
      {isAddingSocial && (
        <AddSocial
          setIsAddingSocial={setIsAddingSocial}
          reloadSocials={loadSocials}
        />
      )}
      {isEditingSocial && (
        <EditSocial
          selectedSocial={selectedSocial}
          setIsEditingSocial={setIsEditingSocial}
          reloadSocials={loadSocials}
        />
      )}
    </div>
  );
};

export default Dashboard;
