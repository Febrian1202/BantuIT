-- Add migration script here
CREATE INDEX idx_users_role ON users(role_id);
CREATE INDEX idx_users_department ON users(department_id);
CREATE INDEX idx_users_status ON users(status);
CREATE INDEX idx_tokens_token ON personal_access_tokens(token);
CREATE INDEX idx_tokens_user ON personal_access_tokens(tokenable_id);
CREATE INDEX idx_assets_status ON assets(status);
CREATE INDEX idx_asset_assignments_active ON asset_assignments(asset_id, released_at);
CREATE INDEX idx_tickets_reporter ON tickets(reporter_id);
CREATE INDEX idx_tickets_technician ON tickets(technician_id);
CREATE INDEX idx_tickets_status ON tickets(status_id);
CREATE INDEX idx_tickets_priority ON tickets(priority_id);
CREATE INDEX idx_tickets_sla ON tickets(sla_breached, sla_deadline);
CREATE INDEX idx_tickets_status_technician ON tickets(status_id, technician_id);
CREATE INDEX idx_notifications_user_read ON notifications(user_id, is_read);
CREATE INDEX idx_audit_logs_module ON audit_logs(module, module_id);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at);
CREATE INDEX idx_knowledge_articles_slug ON knowledge_articles(slug);
