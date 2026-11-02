import { request } from '../core/client';

/** A change of the content model someone asked for on a Setup page. */
export type ChangeRequest = {
	id: number;
	/** the path of the Setup page */
	page: string;
	/** what the page shows, like `Entry types › Page` */
	context: string;
	text: string;
	/** the name or email of who wrote it */
	user: string | null;
	createdAt: number;
	done: boolean;
};

/** All requests, the open ones first, for admins. */
export function listRequests(): Promise<ChangeRequest[]> {
	return request('GET', '/change-requests');
}

/** A request of anyone, admins get a mail if mails are set up. */
export function createRequest(req: {
	page: string;
	context: string;
	text: string;
}): Promise<ChangeRequest> {
	return request('POST', '/change-requests', req);
}

export function setRequestDone(id: number, done: boolean): Promise<void> {
	return request('PUT', `/change-requests/${id}`, { done });
}

/** How the cms sends mails: its SMTP server, or none (mails are logged). */
export type MailStatus = {
	configured: boolean;
	server: string | null;
	from: string;
};

export function mailStatus(): Promise<MailStatus> {
	return request('GET', '/mail');
}

/** A test mail to oneself. */
export function sendTestMail(): Promise<void> {
	return request('POST', '/mail/test');
}
