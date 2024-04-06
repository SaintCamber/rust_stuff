-- Your SQL goes here
CREATE TRIGGER prevent_conflicting_reservation
BEFORE INSERT ON reservations
BEGIN
    SELECT RAISE(ABORT, 'Conflicting reservation exists for this property and time frame')
    FROM reservations
    WHERE (property_id = NEW.property_id)
    AND NOT (NEW.end_date <= start_date OR NEW.start_date >= end_date);
END;