import React from "react";

const HeaderExperience = ({ setIsAdding }) => {
  return (
    <header>
      <h3>Experience</h3>
      <div style={{ marginTop: "30px", marginBottom: "18px" }}>
        <button onClick={() => setIsAdding(true)}>Add Experience</button>
      </div>
    </header>
  );
};

const HeaderProject = ({ setIsAddingProject }) => {
  return (
    <header>
      <h3>Project</h3>
      <div style={{ marginTop: "30px", marginBottom: "18px" }}>
        <button onClick={() => setIsAddingProject(true)}>Add Project</button>
      </div>
    </header>
  );
};

const HeaderSkill = ({ setIsAddingSkill }) => {
  return (
    <header>
      <h3>Skill</h3>
      <div style={{ marginTop: "30px", marginBottom: "18px" }}>
        <button onClick={() => setIsAddingSkill(true)}>Add Skill</button>
      </div>
    </header>
  );
};

const HeaderSocial = ({ setIsAddingSocial }) => {
  return (
    <header>
      <h3>Social</h3>
      <div style={{ marginTop: "30px", marginBottom: "18px" }}>
        <button onClick={() => setIsAddingSocial(true)}>Add Social</button>
      </div>
    </header>
  );
};

export { HeaderExperience, HeaderProject, HeaderSkill, HeaderSocial };
