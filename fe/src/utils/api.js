// utils/api.js

const BASE_URL = "http://127.0.0.1:8000";

export async function apiRequest(
  endpoint,
  method = "GET",
  body = null,
  token = null,
) {
  const headers = {
    "Content-Type": "application/json",
    Accept: "application/json",
  };

  if (token) {
    headers["Authorization"] = `Bearer ${token}`;
  }

  const config = { method, headers };

  if (body) config.body = JSON.stringify(body);

  const response = await fetch(`${BASE_URL}${endpoint}`, config);

  if (!response.ok) {
    const error = await response.text();
    throw new Error(error || "API request failed");
  }

  return await response.json();
}

export const getCustomerDetail = (id, token = null) =>
  apiRequest(`/customer/${id}`, "GET", null, token);

export const updateCustomerById = (id, data, token) =>
  apiRequest(`/customer/${id}`, "PATCH", data, token);

export const getExperienceDetail = (id, token = null) =>
  apiRequest(`/experience/${id}`, "GET", null, token);

export const updateExperienceById = (id, data, token) =>
  apiRequest(`/experience/${id}`, "PATCH", data, token);

export const deleteExperienceById = (id, token) =>
  apiRequest(`/experience/${id}`, "DELETE", null, token);

export async function addExperience(payload, token) {
  return await apiRequest("/experience", "POST", payload, token);
}

export const getProjectDetail = (id, token = null) =>
  apiRequest(`/project/${id}`, "GET", null, token);

export const updateProjectById = (id, data, token) =>
  apiRequest(`/project/${id}`, "PATCH", data, token);

export const deleteProjectById = (id, token) =>
  apiRequest(`/project/${id}`, "DELETE", null, token);

export async function addProject(payload, token) {
  return await apiRequest("/project", "POST", payload, token);
}

export const getSkillDetail = (id, token = null) =>
  apiRequest(`/skill/${id}`, "GET", null, token);

export const updateSkillById = (id, data, token) =>
  apiRequest(`/skill/${id}`, "PATCH", data, token);

export const deleteSkillById = (id, token) =>
  apiRequest(`/skill/${id}`, "DELETE", null, token);

export async function addSkill(payload, token) {
  return await apiRequest("/skill", "POST", payload, token);
}

export const getSocialDetail = (id, token = null) =>
  apiRequest(`/social/${id}`, "GET", null, token);

export const updateSocialById = (id, data, token) =>
  apiRequest(`/social/${id}`, "PATCH", data, token);

export const deleteSocialById = (id, token) =>
  apiRequest(`/social/${id}`, "DELETE", null, token);

export async function addSocial(payload, token) {
  return await apiRequest("/social", "POST", payload, token);
}
