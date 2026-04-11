import { useEffect, useRef, useState } from 'react';
import FullCalendar from '@fullcalendar/react';
import dayGridPlugin from '@fullcalendar/daygrid';
import timeGridPlugin from '@fullcalendar/timegrid';
import interactionPlugin from '@fullcalendar/interaction';
import esLocale from '@fullcalendar/core/locales/es';
import { activitiesApi, type Activity, type CalendarEvent, type ExpiringOpportunity } from '../../lib/api/activities';
import ActivitySidebar from './ActivitySidebar';
import ActivityModal from './ActivityModal';
import { format, startOfMonth, endOfMonth } from 'date-fns';

const TYPE_COLORS: Record<string, string> = {
  call:       '#3b82f6',
  email:      '#6b7280',
  visit:      '#10b981',
  whatsapp:   '#25d366',
  meeting:    '#8b5cf6',
  test_drive: '#f97316',
  delivery:   '#eab308',
};

export default function CalendarMain() {
  const calendarRef = useRef<FullCalendar>(null);
  const [events, setEvents] = useState<CalendarEvent[]>([]);
  const [upcomingActivities, setUpcomingActivities] = useState<Activity[]>([]);
  const [overdueCount, setOverdueCount] = useState(0);
  const [expiringOpportunities, setExpiringOpportunities] = useState<ExpiringOpportunity[]>([]);
  const [selectedActivity, setSelectedActivity] = useState<Activity | null>(null);
  const [showModal, setShowModal] = useState(false);
  const [newActivityDate, setNewActivityDate] = useState<string | null>(null);

  const loadCalendar = async (from: Date, to: Date) => {
    try {
      const res = await activitiesApi.calendar(
        format(from, "yyyy-MM-dd'T'HH:mm:ssxxx"),
        format(to, "yyyy-MM-dd'T'HH:mm:ssxxx")
      );
      setEvents(res.events);
      setOverdueCount(res.overdue_count);
      setExpiringOpportunities(res.expiring_opportunities ?? []);
    } catch (err) {
      console.error('Error cargando calendario:', err);
    }
  };

  const loadUpcoming = async () => {
    try {
      const activities = await activitiesApi.upcoming();
      setUpcomingActivities(activities);
    } catch (err) {
      console.error('Error cargando proximas actividades:', err);
    }
  };

  useEffect(() => {
    const now = new Date();
    loadCalendar(startOfMonth(now), endOfMonth(now));
    loadUpcoming();
  }, []);

  const handleDateClick = (info: { dateStr: string }) => {
    setNewActivityDate(info.dateStr);
    setSelectedActivity(null);
    setShowModal(true);
  };

  const handleEventClick = (info: { event: { id: string; extendedProps: Activity } }) => {
    setSelectedActivity(info.event.extendedProps);
    setNewActivityDate(null);
    setShowModal(true);
  };

  const handleDatesSet = (info: { start: Date; end: Date }) => {
    loadCalendar(info.start, info.end);
  };

  const handleModalClose = () => {
    setShowModal(false);
    setSelectedActivity(null);
    setNewActivityDate(null);
  };

  const handleActivitySaved = () => {
    handleModalClose();
    const cal = calendarRef.current?.getApi();
    if (cal) {
      const view = cal.view;
      loadCalendar(view.activeStart, view.activeEnd);
    }
    loadUpcoming();
  };

  const calendarEvents = events.map(e => ({
    id: e.id,
    title: e.title,
    start: e.start,
    end: e.end,
    backgroundColor: TYPE_COLORS[e.type] ?? '#6b7280',
    borderColor: TYPE_COLORS[e.type] ?? '#6b7280',
    extendedProps: e.extendedProps,
    classNames: e.status === 'completed' ? ['opacity-60'] : [],
  }));

  return (
    <div className="flex gap-4 h-full">
      {/* Calendario principal */}
      <div className="flex-1 bg-white rounded-xl border border-gray-200 shadow-sm p-4 overflow-hidden">
        <FullCalendar
          ref={calendarRef}
          plugins={[dayGridPlugin, timeGridPlugin, interactionPlugin]}
          initialView="dayGridMonth"
          locale={esLocale}
          headerToolbar={{
            left: 'prev,next today',
            center: 'title',
            right: 'dayGridMonth,timeGridWeek,timeGridDay',
          }}
          events={calendarEvents}
          editable={true}
          selectable={true}
          dateClick={handleDateClick}
          eventClick={handleEventClick}
          datesSet={handleDatesSet}
          eventDrop={async (info) => {
            try {
              await activitiesApi.reschedule(
                info.event.id,
                info.event.startStr,
                info.event.endStr ?? info.event.startStr
              );
            } catch {
              info.revert();
            }
          }}
          height="100%"
          buttonText={{ today: 'Hoy', month: 'Mes', week: 'Semana', day: 'Dia' }}
        />
      </div>

      {/* Sidebar lateral */}
      <ActivitySidebar
        upcomingActivities={upcomingActivities}
        overdueCount={overdueCount}
        expiringOpportunities={expiringOpportunities}
        onActivityClick={(activity) => {
          setSelectedActivity(activity);
          setShowModal(true);
        }}
        onNewActivity={() => {
          setSelectedActivity(null);
          setNewActivityDate(new Date().toISOString());
          setShowModal(true);
        }}
      />

      {/* Modal crear/ver actividad */}
      {showModal && (
        <ActivityModal
          activity={selectedActivity}
          defaultDate={newActivityDate}
          onClose={handleModalClose}
          onSaved={handleActivitySaved}
        />
      )}
    </div>
  );
}
