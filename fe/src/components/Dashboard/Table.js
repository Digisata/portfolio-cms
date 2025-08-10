// ExperienceTable.js
import React from "react";

const ExperienceTable = ({ experiences, handleEdit, handleDelete, error }) => {
  return (
    <div className="contain-table">
      {error && <p style={{ color: "red" }}>{error}</p>}
      <table className="striped-table">
        <thead>
          <tr>
            <th>No.</th>
            <th>Company</th>
            <th>Work Type</th>
            <th>Location</th>
            <th>Position</th>
            <th>Start Date</th>
            <th>End Date</th>
            <th>Is Present</th>
            <th>Order</th>
            <th colSpan={2} className="text-center">
              Actions
            </th>
          </tr>
        </thead>
        <tbody>
          {experiences.length > 0 ? (
            experiences.map((exp, i) => (
              <tr key={exp._id}>
                <td>{i + 1}</td>
                <td>{exp.company}</td>
                <td>{exp.work_type}</td>
                <td>{exp.location}</td>
                <td>{exp.position}</td>
                <td>{exp.start_date}</td>
                <td>{exp.end_date}</td>
                <td>{exp.is_present ? "✅" : "❌"}</td>
                <td>{exp.order}</td>
                <td className="text-right">
                  <button
                    onClick={() => handleEdit(exp._id)}
                    className="button muted-button"
                  >
                    Edit
                  </button>
                </td>
                <td className="text-left">
                  <button
                    onClick={() => handleDelete(exp._id)}
                    className="button muted-button"
                  >
                    Delete
                  </button>
                </td>
              </tr>
            ))
          ) : (
            <tr>
              <td colSpan={10}>No Experience Found</td>
            </tr>
          )}
        </tbody>
      </table>
    </div>
  );
};

const ProjectTable = ({
  projects,
  handleEditProject,
  handleDeleteProject,
  error,
}) => {
  return (
    <div className="contain-table">
      {error && <p style={{ color: "red" }}>{error}</p>}
      <table className="striped-table">
        <thead>
          <tr>
            <th>No.</th>
            <th>Name</th>
            <th>Description</th>
            <th>Link</th>
            <th>Photo Link</th>
            <th>Order</th>
            <th>Stack</th>
            <th colSpan={2} className="text-center">
              Actions
            </th>
          </tr>
        </thead>
        <tbody>
          {projects.length > 0 ? (
            projects.map((project, i) => (
              <tr key={project._id}>
                <td>{i + 1}</td>
                <td>{project.name}</td>
                <td>{project.description}</td>
                <td>
                  <a
                    href={project.link}
                    target="_blank"
                    rel="noopener noreferrer"
                  >
                    {project.link}
                  </a>
                </td>
                <td>
                  <a
                    href={project.photo_link}
                    target="_blank"
                    rel="noopener noreferrer"
                  >
                    {project.photo_link}
                  </a>
                </td>
                <td>{project.order}</td>
                <td>{project.stack && project.stack.join(", ")}</td>
                <td className="text-right">
                  <button
                    onClick={() => handleEditProject(project._id)}
                    className="button muted-button"
                  >
                    Edit
                  </button>
                </td>
                <td className="text-left">
                  <button
                    onClick={() => handleDeleteProject(project._id)}
                    className="button muted-button"
                  >
                    Delete
                  </button>
                </td>
              </tr>
            ))
          ) : (
            <tr>
              <td colSpan={8}>No Projects Found</td>
            </tr>
          )}
        </tbody>
      </table>
    </div>
  );
};

const SkillTable = ({ skills, handleEditSkill, handleDeleteSkill, error }) => {
  return (
    <div className="contain-table">
      {error && <p style={{ color: "red" }}>{error}</p>}
      <table className="striped-table">
        <thead>
          <tr>
            <th>No.</th>
            <th>Name</th>
            <th>Order</th>
            <th colSpan={2} className="text-center">
              Actions
            </th>
          </tr>
        </thead>
        <tbody>
          {skills.length > 0 ? (
            skills.map((skill, i) => (
              <tr key={skill._id}>
                <td>{i + 1}</td>
                <td>{skill.name}</td>
                <td>{skill.order}</td>
                <td className="text-right">
                  <button
                    onClick={() => handleEditSkill(skill._id)}
                    className="button muted-button"
                  >
                    Edit
                  </button>
                </td>
                <td className="text-left">
                  <button
                    onClick={() => handleDeleteSkill(skill._id)}
                    className="button muted-button"
                  >
                    Delete
                  </button>
                </td>
              </tr>
            ))
          ) : (
            <tr>
              <td colSpan={8}>No Skills Found</td>
            </tr>
          )}
        </tbody>
      </table>
    </div>
  );
};

const SocialTable = ({
  socials,
  handleEditSocial,
  handleDeleteSocial,
  error,
}) => {
  return (
    <div className="contain-table">
      {error && <p style={{ color: "red" }}>{error}</p>}
      <table className="striped-table">
        <thead>
          <tr>
            <th>No.</th>
            <th>Name</th>
            <th>Link</th>
            <th>Order</th>
            <th colSpan={2} className="text-center">
              Actions
            </th>
          </tr>
        </thead>
        <tbody>
          {socials.length > 0 ? (
            socials.map((social, i) => (
              <tr key={social._id}>
                <td>{i + 1}</td>
                <td>{social.name}</td>
                <td>
                  <a
                    href={social.link}
                    target="_blank"
                    rel="noopener noreferrer"
                  >
                    {social.link}
                  </a>
                </td>
                <td>{social.order}</td>
                <td className="text-right">
                  <button
                    onClick={() => handleEditSocial(social._id)}
                    className="button muted-button"
                  >
                    Edit
                  </button>
                </td>
                <td className="text-left">
                  <button
                    onClick={() => handleDeleteSocial(social._id)}
                    className="button muted-button"
                  >
                    Delete
                  </button>
                </td>
              </tr>
            ))
          ) : (
            <tr>
              <td colSpan={8}>No Socials Found</td>
            </tr>
          )}
        </tbody>
      </table>
    </div>
  );
};

export { ExperienceTable, ProjectTable, SkillTable, SocialTable };
